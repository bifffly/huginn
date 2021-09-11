use gtk::glib;
use gtk::prelude::*;
use gtk::{IconSize, MenuBar, MenuItem, Orientation, ReliefStyle, Widget};

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
pub struct Notebook {
    pub notebook: gtk::Notebook,
    tabs: Vec<gtk::Box>   
}

impl Notebook {
    pub fn new() -> Self {
        Self {
            notebook: gtk::Notebook::new(),
            tabs: Vec::new()
        }
    }

    pub fn create_tab(&mut self, title: &str, widget: Widget) -> u32 {
        let close_image = gtk::Image::from_icon_name(Some("window-close"), IconSize::Button);
        let button = gtk::Button::new();
        let label = gtk::Label::new(Some(title));
        let tab = gtk::Box::new(Orientation::Horizontal, 0);

        button.set_relief(ReliefStyle::None);
        button.add(&close_image);

        tab.pack_start(&label, false, false, 0);
        tab.pack_start(&button, false, false, 0);
        tab.show_all();

        let index = self.notebook.append_page(&widget, Some(&tab));

        button.connect_clicked(glib::clone!(@weak self.notebook as notebook => move |_| {
            let index = notebook.page_num(&widget).expect("");
            notebook.remove_page(Some(index));
        }));
       
        self.tabs.push(tab);

        return index;
    }
}
