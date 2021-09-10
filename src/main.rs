use gtk::prelude::*;

fn main() {
    let app = gtk::Application::new(Some("com.odin-project.huginn"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Huginn");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);

    window.show_all();
}
