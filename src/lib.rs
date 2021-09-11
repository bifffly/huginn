use gtk::prelude::*;
use gtk::{
    Align, Box, Button, IconSize, Image, Label, Notebook, 
    Orientation, PositionType, SearchEntry, ScrolledWindow
};

// TOOLBAR
pub fn create_toolbar() -> Box {
    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(0)
        .build();

    let back_img = Image::builder()
        .icon_name("go-previous")
        .icon_size(IconSize::Menu)
        .build();
    let back_button = Button::new();
    back_button.add(&back_img);
    hbox.pack_start(&back_button, false, false, 0);

    let next_img = Image::builder()
        .icon_name("go-next")
        .icon_size(IconSize::Menu)
        .build();
    let next_button = Button::new();
    next_button.add(&next_img);
    hbox.pack_start(&next_button, false, false, 0);

    let search_entry = SearchEntry::new();
    hbox.pack_start(&search_entry, true, true, 0);

    let search_img = Image::builder()
        .icon_name("edit-find")
        .icon_size(IconSize::Menu)
        .build();
    let search_button = Button::new();
    search_button.add(&search_img);
    hbox.pack_start(&search_button, false, false, 0);

    return hbox;
}

// NOTEBOOK
pub fn create_notebook() -> Notebook {
    let notebook = Notebook::builder()
        .tab_pos(PositionType::Top)
        .show_border(true)
        .build();
    let contentbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .build();
    let content = ScrolledWindow::builder()
        .valign(Align::Fill)
        .build();
    contentbox.pack_start(&content, false, false, 0);

    notebook.append_page(&contentbox, Some(&Label::new(Some("Welcome to Huginn"))));
    return notebook;
}
