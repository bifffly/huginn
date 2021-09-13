use vgtk::{Component, gtk, UpdateAction, VNode};
use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;

use crate::*;

impl Component for HuginnWindow {
    type Message = Msg;
    type Properties = Self;

    fn create(props: Self) -> Self {
        return props;
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        return UpdateAction::Render;
    }

    fn update(&mut self, msg: Msg) -> UpdateAction<Self> {
        match msg {
            Msg::BACK => {
                println!("BACK");
            },
            Msg::NEXT => {
                println!("NEXT");
            },
            Msg::SEARCH_CHANGE {url} => {
                self.url = url.clone();
                println!("SEARCH_CHANGE {}", self.url);
            },
            Msg::SEARCH_SEND => {
                println!("SEARCH_SEND");
                return UpdateAction::Render;
            },
            Msg::EXIT => {
                println!("EXIT");
            }
        };
        return UpdateAction::None;
    }

    fn view(&self) -> VNode<Self> {
        let url = self.url.clone();
        gtk! {
            <Application::new_unwrap(None, ApplicationFlags::empty())>
                <Window default_width=800 default_height=600 on destroy=|_| Msg::EXIT> 
                    <Box orientation=Orientation::Vertical spacing=10>
                        <@HuginnToolbar
                            on_back=|_| Msg::BACK
                            on_next=|_| Msg::NEXT
                            on_search_change=|url| Msg::SEARCH_CHANGE {url}
                            on_search_send=|url| Msg::SEARCH_SEND
                        />
                        <@HuginnDisplay url=url/>
                    </Box>
                </Window>
            </Application>
        }
    }
}
