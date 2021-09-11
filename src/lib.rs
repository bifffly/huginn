use gtk::prelude::*;
use gtk::{
    Align, Box, Button, IconSize, Image, Label,
    Orientation, SearchEntry, ScrolledWindow
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
        .icon_name("system-search-symbolic")
        .icon_size(IconSize::Menu)
        .build();
    let search_button = Button::new();
    search_button.add(&search_img);
    hbox.pack_end(&search_button, false, false, 0);

    return hbox;
}

// RENDER AREA
pub fn create_render_area() -> ScrolledWindow {
    let lorem_ipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec diam elit, tristique eget finibus vel, vulputate at eros. Suspendisse euismod lacinia velit, et vestibulum nibh ultricies non. Sed blandit pellentesque magna, non aliquam dolor. Praesent congue velit vitae congue congue. Proin sed sem vel ex tincidunt eleifend et et urna. Mauris sodales dictum tristique. Nam rhoncus quis arcu et lacinia. Duis rutrum faucibus ultricies.";
    let label = Label::builder()
        .label(lorem_ipsum)
        .wrap(true)
        .xalign(0.0)
        .yalign(0.0)
        .build();
    let scrolled_window = ScrolledWindow::builder()
        .child(&label)
        .valign(Align::Fill)
        .build();
    return scrolled_window;
}
