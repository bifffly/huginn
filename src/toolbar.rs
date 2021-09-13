use vgtk::{Component, gtk, UpdateAction, VNode};
use vgtk::lib::gtk::*;

use crate::*;

impl Component for HuginnToolbar {
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
                self.on_back.send(());
            },
            Msg::NEXT => {
                self.on_next.send(());
            },
            Msg::SEARCH_CHANGE {url} => {
                let url_clone = url.clone();
                self.on_search_change.send(url_clone);
            },
            Msg::SEARCH_SEND => {
                self.on_search_send.send(());
            }
            _ => {}
        };
        return UpdateAction::None;
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box orientation=Orientation::Horizontal spacing=10>
                <ButtonBox orientation=Orientation::Horizontal layout=ButtonBoxStyle::Expand>
                    <Button image="go-previous" on clicked=|_| Msg::BACK/>
                    <Button image="go-next" on clicked=|_| Msg::NEXT/>
                </ButtonBox>
                <SearchEntry hexpand=true on search_changed=|entry| {
                    Msg::SEARCH_CHANGE {
                        url: entry.get_text().as_str().to_string()
                    }
                }/>
                <Button image="system-search-symbolic" on clicked=|_| Msg::SEARCH_SEND/> 
            </Box>
        }
    }
}

