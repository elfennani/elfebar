use gdk::{Display, Screen};
use gtk::{
    traits::{BuildableExt, ContainerExt, CssProviderExt, GtkWindowExt, WidgetExt},
    CssProvider,
};
use gtk_layer_shell::{self, Layer};

fn main() {
    gtk::init().unwrap();
    let window = gtk::Window::builder()
        .type_(gtk::WindowType::Toplevel)
        .build();
    window.set_size_request(1920, 32);

    gtk_layer_shell::init_for_window(&window);

    gtk_layer_shell::set_layer(&window, Layer::Top);
    gtk_layer_shell::auto_exclusive_zone_enable(&window);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);

    let file = std::fs::File::open("/home/elfennani/rust/gui-test/src/style.css")
        .expect("Failed to open file");

    let provider = CssProvider::new();
    provider
        .load_from_path("/home/elfennani/rust/gui-test/src/style.css")
        .unwrap();
    // Add the provider to the default screen
    gtk::StyleContext::add_provider_for_screen(
        &Screen::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let label = gtk::Label::new(Some("Hello World"));
    window.set_child(Some(&label));

    window.show_all();
    gtk::main();
}
