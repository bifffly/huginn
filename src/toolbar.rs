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
            Msg::BACK => {self.back_clicked.send(());},
            Msg::NEXT => {self.next_clicked.send(());},
            Msg::SEARCH => {self.search_clicked.send(());},
            _ => {}
        }
        return UpdateAction::None;
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box orientation=Orientation::Horizontal spacing=10>
                <Button image="go-previous" on clicked=|_| Msg::BACK/>
                <Button image="go-next" on clicked=|_| Msg::NEXT/>
                <SearchEntry hexpand=true/>
                <Button image="system-search-symbolic" on clicked=|_| Msg::SEARCH/>
            </Box>
        }
    }
}

