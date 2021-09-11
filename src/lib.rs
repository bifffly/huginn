use gtk::prelude::*;
use gtk::{
    Align, Box, Button, IconSize, Image, Label,
    Orientation, SearchEntry, ScrolledWindow
};

pub struct Huginn {
    pub toolbar: Toolbar,
    pub render_area: ScrolledWindow
}

impl Huginn {
    pub fn new() -> Self {
        let toolbar = create_toolbar();
        let render_area = create_render_area(&toolbar.search_entry, &toolbar.search_button);
        return Huginn {toolbar, render_area};
    }
}

pub struct Toolbar {
    pub toolbar: Box,
    search_entry: SearchEntry,
    search_button: Button
}

fn create_toolbar() -> Toolbar {
    let toolbar = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(0)
    .build();
    let back_img = Image::builder()
        .icon_name("go-previous")
        .icon_size(IconSize::Menu)
        .build();
    let back_button = Button::new();
    back_button.add(&back_img);
    toolbar.pack_start(&back_button, false, false, 0);
    
    let next_img = Image::builder()
        .icon_name("go-next")
        .icon_size(IconSize::Menu)
        .build();
    let next_button = Button::new();
    next_button.add(&next_img);
    toolbar.pack_start(&next_button, false, false, 0);

    let search_entry = SearchEntry::new();
    toolbar.pack_start(&search_entry, true, true, 0);
    
    let search_img = Image::builder()
        .icon_name("system-search-symbolic")
        .icon_size(IconSize::Menu)
        .build();
    let search_button = Button::new();
    search_button.add(&search_img);
    toolbar.pack_end(&search_button, false, false, 0);

    return Toolbar {toolbar, search_entry, search_button};
}

fn create_render_area(search_entry: &SearchEntry, search_button: &Button) -> ScrolledWindow {
    let search_entry_clone = search_entry.clone();
    let label = Label::builder()
        .label("")
        .wrap(true)
        .xalign(0.0)
        .yalign(0.0)
        .build();
    let label_clone = label.clone();
    search_button.connect_clicked(move |_| {
        label.set_text(&search_entry_clone.text().to_string());
    });

    let render_area = ScrolledWindow::builder()
        .child(&label_clone)
        .valign(Align::Fill)
        .build();

    return render_area;
}
