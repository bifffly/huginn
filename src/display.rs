use vgtk::{Component, gtk, UpdateAction, VNode};
use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;

use crate::*;

impl Component for HuginnDisplay {
    type Message = Msg;
    type Properties = ();

    fn update(&mut self, msg: Msg) -> UpdateAction<Self> {
        match msg {
            Msg::SEARCH => {
                println!("Searched");
                return UpdateAction::Render;
            },
            _ => {return UpdateAction::None;}
        }
    }
}
