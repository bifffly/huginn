use vgtk::{Component, gtk, UpdateAction, VNode};
use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;

use crate::*;

impl Component for HuginnWindow {
    type Message = Msg;
    type Properties = ();

    fn update(&mut self, msg: Msg) -> UpdateAction<Self> {
        match msg {
            Msg::BACK => {
                println!("Back");
                return UpdateAction::None;
            },
            Msg::NEXT => {
                println!("Next");
                return UpdateAction::None;
            },
            Msg::SEARCH => {
                println!("Search");
                return UpdateAction::None;
            },
            Msg::EXIT => {
                println!("Exit");
                return UpdateAction::None;
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Application::new_unwrap(None, ApplicationFlags::empty())>
                <Window default_width=800 default_height=600 on destroy=|_| Msg::EXIT> 
                    <Box orientation=Orientation::Vertical spacing=10>
                        <@HuginnToolbar
                            back_clicked=|_| Msg::BACK
                            next_clicked=|_| Msg::NEXT
                            search_clicked=|_| Msg::SEARCH
                        />
                        <Label label="Huginn"/>
                    </Box>
                </Window>
            </Application>
        }
    }
}
