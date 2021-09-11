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
    // wbox.pack_start(&menubar, false, false, 0);

    let mut notebook = Notebook::new();
    let title = "Welcome to Huginn";
    let label = gtk::Label::new(Some(&*title));
    notebook.create_tab(&title, label.upcast());
    // wbox.pack_start(&notebook.notebook, false, false, 0);

    window.add(&notebook.notebook);

    window.show_all();
}
