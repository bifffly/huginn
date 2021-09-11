use gtk::glib;
use gtk::prelude::*;
use gtk::{Align, MenuBar, MenuItem, Orientation};

// MENUBAR
pub fn create_menubar(window: &gtk::ApplicationWindow) -> gtk::Box {
    let menubox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let menubar = MenuBar::new();
    let file = MenuItem::with_label("File");
    let quit = MenuItem::with_label("Quit");
    quit.connect_activate(glib::clone!(@weak window => move |_| {
        window.close();
    }));
    menubar.append(&file);
    menubar.append(&quit);
    menubox.pack_start(&menubar, false, false, 0);
    return menubox;
}

// NOTEBOOK
pub fn create_notebook() -> gtk::Notebook {
    let notebook = gtk::Notebook::builder()
        .tab_pos(gtk::PositionType::Top)
        .show_border(true)
        .build();
    let contentbox = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .build();
    let content = gtk::ScrolledWindow::builder()
        .valign(Align::Fill)
        .build();
    contentbox.pack_start(&content, false, false, 0);
    notebook.append_page(&contentbox, Some(&gtk::Label::new(Some("Welcome to Huginn"))));
    return notebook;
}
