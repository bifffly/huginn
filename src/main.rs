use gtk::prelude::*;

use huginn::*;

fn main() {
    let app = gtk::Application::new(Some("com.odin-project.huginn"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Huginn");
    window.set_default_size(640, 480);
    window.set_position(gtk::WindowPosition::Center);

    let wbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let menubar: gtk::Box = create_menubar(&window);
    wbox.pack_start(&menubar, false, false, 0);

    let notebook = create_notebook();
    wbox.pack_start(&notebook, false, false, 0);

    window.add(&wbox);
    window.show_all();
}
