use vgtk::{Component, gtk, UpdateAction, VNode};
use vgtk::lib::gtk::*;

use crate::*;

impl Component for HuginnDisplay {
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
            _ => {}
        };
        return UpdateAction::None;
    }

    fn view(&self) -> VNode<Self> {
        let url = self.url.clone();
        gtk! {
            <ScrolledWindow>
                <Label label=url/>
            </ScrolledWindow>
        }
    }
}
