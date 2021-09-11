use dns_lookup::lookup_host;
use gtk::prelude::*;
use gtk::{
    Align, Box, Button, IconSize, Image, Label,
    Orientation, SearchEntry, ScrolledWindow
};
use std::io::{Read, Write};
use std::net::TcpStream;
use url::Url;

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
        let address = &search_entry_clone.text().to_string();
        let response = send_request(address);
        label.set_text(&response);
    });

    let render_area = ScrolledWindow::builder()
        .child(&label_clone)
        .valign(Align::Fill)
        .build();

    return render_area;
}

fn send_request(address: &String) -> String {
    let url = Url::parse(address).unwrap();
    let ipaddr = lookup_host(url.host_str().unwrap()).unwrap()[0];
    let mut stream = TcpStream::connect(ipaddr.to_string() + &":1866").unwrap();
    let writestr = "odin\tpull\t".to_owned() + &url.path();
    stream.write(writestr.as_bytes());
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).unwrap();
    let mut v = vec![];
    for byte in buffer {
        match byte {
            0 => break,
            _ => v.push(byte),
        }
    }
    return String::from_utf8_lossy(&v[..]).to_string();
}
