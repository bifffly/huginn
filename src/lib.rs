use flume::{Receiver, Sender, SendError};
use futures::task::LocalSpawnExt;
use gtk::glib::MainContext;
use gtk::prelude::*;
use gtk::{
    Box, Button, IconSize, Image, Label, Notebook,
    Orientation, PackType, SearchEntry, ScrolledWindow
};

#[derive(Debug)]
pub enum Msg {
    NEWTAB
}

pub struct Huginn {
    pub toolbar: Toolbar,
    pub notebook: NotebookArea,
    pub sender: Sender<Msg>
}

impl Huginn {
    pub fn new() -> Self {
        let toolbar = Toolbar::new();
        let toolbar_clone = toolbar.clone();
        
        let mut notebook = NotebookArea::new();
        let notebook_clone = notebook.clone();
        notebook.add_tab();
        
        let (sender, receiver): (Sender<Msg>, Receiver<Msg>) = flume::unbounded();
        let sender_clone = sender.clone();

        let mut this = Self {
            toolbar: toolbar_clone, 
            notebook: notebook_clone, 
            sender: sender_clone
        };
        this.bind();
        let handle = MainContext::default().spawn_local_with_handle(async move {
            while let Ok(msg) = receiver.recv_async().await {
                this.handle_msg(msg);
            }
        }).unwrap();

        return Self {toolbar, notebook, sender};
    }

    pub fn handle_msg(&mut self, msg: Msg) {
        match msg {
            Msg::NEWTAB => self.notebook.add_tab(),
        }
    }

    pub fn bind(&mut self) {
        let sender_clone = self.sender.clone();
        self.notebook.new_tab_button.connect_clicked(move |_| {
            println!("NEWTAB");
            let res = sender_clone.send(Msg::NEWTAB);
        });
    }
}

#[derive(Clone)]
pub struct Toolbar {
    pub toolbar: Box,
    search_entry: SearchEntry,
    search_button: Button
}

impl Toolbar {
    pub fn new() -> Self {
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
}

#[derive(Clone)]
pub struct NotebookArea {
    pub notebook: Notebook,
    tabs: Vec<ScrolledWindow>,
    new_tab_button: Button
}

impl NotebookArea {
    pub fn new() -> Self {
        let notebook = Notebook::new();
        let new_tab_img = Image::builder()
            .icon_name("document-new-symbolic")
            .icon_size(IconSize::Menu)
            .build();
        let new_tab_button = Button::new();

        let tabs: Vec<ScrolledWindow> = vec![];

        new_tab_button.add(&new_tab_img);
        notebook.set_action_widget(&new_tab_button, PackType::End);
        new_tab_button.show_all();

        return NotebookArea {notebook, tabs, new_tab_button};
    }

    pub fn add_tab(&mut self) {
        let window = ScrolledWindow::builder()
            .build();
        let label = Label::new(Some("New tab"));
        let tab_n = self.notebook.append_page(&window, Some(&label));
        self.tabs.push(window);
        self.notebook.set_current_page(Some(tab_n));
    }
}

