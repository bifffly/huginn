use gtk::Orientation;
use gtk::prelude::*;
use relm::{connect, init, Relm, Widget};
use relm_derive::{Msg, widget};

use crate::window::*;

#[derive(Msg)]
pub enum ToolbarMsg {
    BACK,
    NEXT,
    SEARCH(String)
}

pub struct ToolbarModel {
    relm: Relm<HuginnToolbar>,
    address: String,
    title: String
}

#[widget]
impl Widget for HuginnToolbar {
    fn model(relm: &Relm<Self>, _: ()) -> ToolbarModel {
        return ToolbarModel {
            relm: relm.clone(),
            address: String::new(),
            title: String::new()
        };
    }

    fn update(&mut self, msg: ToolbarMsg) {
        match msg {
            ToolbarMsg::SEARCH(address) => {
                // let address = get_search_entry_address(&address);
                // self.model.relm.stream().emit(Msg::SEARCH(address));
            },
            _ => {}
        }
    }

    view! {
        gtk::HeaderBar {
            show_close_button: false,
            custom_title: view! {
                gtk::Entry {
                    text: &self.model.address,
                    input_purpose: gtk::InputPurpose::Url,
                    max_width_chars: 100,
                    activate(entry) => {
                        let text = entry.text();
                        println!("{}", text);
                        ToolbarMsg::SEARCH(text.to_string())
                    }
                }
            },
            gtk::ButtonBox {
                orientation: Orientation::Horizontal,
                layout: gtk::ButtonBoxStyle::Expand,
                gtk::Button {
                    tooltip_text: Some("Back"),
                    image: Some(&gtk::Image::from_icon_name(Some("go-previous"), gtk::IconSize::Menu)),
                    clicked(_) => ToolbarMsg::BACK,
                },
                gtk::Button {
                    tooltip_text: Some("Next"),
                    image: Some(&gtk::Image::from_icon_name(Some("go-next"), gtk::IconSize::Menu)),
                    clicked(_) => ToolbarMsg::NEXT,
                }
            }
        }
    }
}
