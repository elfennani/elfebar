#![allow(unreachable_code, unused)]
use glib::{MainContext, Priority};
use hyprland::{
    data::Client,
    event_listener::EventListener,
    shared::{Address, HyprDataActiveOptional},
};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    cell::RefCell,
    f32::consts::E,
    future::Future,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

use gdk::{gio::GioFutureResult, Screen};
use gtk::prelude::*;
use gtk::{
    traits::{ButtonExt, ContainerExt, CssProviderExt, WidgetExt},
    CssProvider,
};
use gtk_layer_shell::{self, Layer};

fn get_active_window() -> Option<String> {
    let active_window: Option<Client> = {
        let active_window = Client::get_active();

        active_window.unwrap_or(None)
    };

    active_window.map(|w| w.initial_title)
}

#[tokio::main]
async fn main() {
    gtk::init().unwrap();
    let window = gtk::Window::builder()
        .type_(gtk::WindowType::Toplevel)
        .build();
    window.set_size_request(1920, 32);
    let state: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));

    setup_style();
    setup_layer_shell(&window);

    let label = gtk::Label::new(Some(
        get_active_window()
            .unwrap_or("Hyprland".to_string())
            .as_str(),
    ));

    let label_rc = Rc::new(RefCell::new(label.clone()));

    let mut listener = EventListener::new();
    listener.add_active_window_change_handler(move |event| {
        let label_content = get_active_window().unwrap_or("Hyprland".to_string());
        label_rc.borrow_mut().set_label(&label_content);
    });

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.add(&label);
    // give it a css class
    hbox.style_context().add_class("box");
    window.set_child(Some(&hbox));

    window.connect_scroll_event(|_, _| gtk::Inhibit(true));

    window.show_all();

    std::thread::spawn(move || {
        listener.start_listener().unwrap();
    });
    // println!("before");
    // println!("after");

    gtk::main();
}

#[cfg(debug_assertions)]
fn setup_style() {
    // load_css();
    // let provider = Rc::new(RefCell::new(()));
    let provider = Arc::new(Mutex::new(CssProvider::new()));

    let (sender, receiver) = glib::MainContext::channel::<()>(Priority::default());
    let path = Path::new("src/style.css");

    std::thread::spawn(move || {
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

        watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
        for res in rx {
            if let Ok(event) = res {
                if let EventKind::Modify(_) = event.kind {
                    sender.send(()).unwrap();
                }
            }
        }
    });

    load_css(&provider.lock().unwrap());

    receiver.attach(None, move |_| {
        load_css(&*provider.lock().unwrap());
        glib::Continue(true)
    });
}

#[cfg(not(debug_assertions))]
fn setup_style() {
    let provider = Arc::new(Mutex::new(CssProvider::new()));
    load_css(&provider.lock().unwrap());
}

fn load_css(provider: &CssProvider) {
    if provider
        .load_from_path("src/style.css")
        .map_err(|err| println!("{err:?}"))
        .is_ok()
    {
        gtk::StyleContext::add_provider_for_screen(
            &Screen::default().unwrap(),
            provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        return;
    }
}

fn setup_layer_shell(window: &gtk::Window) {
    gtk_layer_shell::init_for_window(window);
    gtk_layer_shell::set_layer(window, Layer::Top);
    gtk_layer_shell::auto_exclusive_zone_enable(window);
    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Top, true);
}
