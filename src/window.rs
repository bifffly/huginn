use gtk::{
    Inhibit, main_quit, Orientation, WindowPosition
};
use gtk::prelude::*;
use relm::{init, Component, Relm, Widget, connect};
use relm_derive::{Msg, widget};

use crate::toolbar::*;

#[derive(Msg)]
pub enum Msg {
    BACK,
    NEXT,
    SEARCH(String),
    TITLE(String),
    EXIT
}

pub struct WindowModel {
    relm: Relm<HuginnWindow>,
    title: String,
    toolbar: Component<HuginnToolbar>
}

#[widget]
impl Widget for HuginnWindow {
    fn model(relm: &Relm<Self>, _: ()) -> WindowModel {
        let toolbar = init::<HuginnToolbar>(()).expect("Couldn't initialize toolbar");
        // let display = init::<HuginnDisplay>(()).expect("Couldn't initialize display");

        // connect!(toolbar @ Msg::SEARCH(address), display, DisplayMsg::LOAD(address.clone()));
        // connect!(toolbar @ ToolbarMsg::BACK, display, DisplayMsg::BACK);
        // connect!(toolbar @ ToolbarMsg::NEXT, display, DisplayMsg::NEXT);

        return WindowModel {
            relm: relm.clone(),
            title: "Huginn".to_string(),
            toolbar,
            // display
        };
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            // Msg::SEARCH(address) => self.model.toolbar.emit(ToolbarMsg::SEARCH(address)),
            Msg::TITLE(title) => self.root().set_title(&title),
            Msg::EXIT => main_quit(),
            _ => (),
        }
    }

    fn init_view(&mut self) {
        let window = self.root();
        window.resize(800, 600);
    }

    view! {
        gtk::ApplicationWindow {
            position: WindowPosition::Center,
            title: &self.model.title,
            gtk::Box {
                orientation: Orientation::Vertical,
                HuginnToolbar{}
            },
            delete_event(_, _) => (Msg::EXIT, Inhibit(false)),
        }
    }
}

