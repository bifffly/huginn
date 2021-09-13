use vgtk::{
    Callback, Component, gtk, UpdateAction, VNode
};
use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;

#[derive(Clone, Debug, Default)]
pub struct Huginn {}

#[derive(Clone, Debug)]
pub enum Msg {
    BACK,
    NEXT,
    SEARCH,
    EXIT
}

impl Component for Huginn {
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

    fn view(&self) -> VNode<Huginn> {
        gtk! {
            <Application::new_unwrap(None, ApplicationFlags::empty())>
                <Window default_width=800 default_height=600 on destroy=|_| Msg::EXIT> 
                    <Box orientation=Orientation::Vertical spacing=10>
                        <@Toolbar
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

#[derive(Clone, Debug, Default)]
pub struct Toolbar {
    pub back_clicked: Callback<()>,
    pub next_clicked: Callback<()>,
    pub search_clicked: Callback<()>
}

impl Component for Toolbar {
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

