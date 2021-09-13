use vgtk::*;

pub mod window;
pub mod toolbar;
pub mod display;
pub mod parser;

#[derive(Clone, Debug)]
pub enum Msg {
    BACK,
    NEXT,
    SEARCH_CHANGE {url: String},
    SEARCH_SEND,
    EXIT
}

#[derive(Clone, Debug, Default)]
pub struct HuginnWindow {
    pub url: String
}

#[derive(Clone, Debug, Default)]
pub struct HuginnToolbar {
    pub url: String,
    pub on_back: Callback<()>,
    pub on_next: Callback<()>,
    pub on_search_change: Callback<String>,
    pub on_search_send: Callback<()>
}

#[derive(Clone, Debug, Default)]
pub struct HuginnDisplay {
    pub url: String
}

#[derive(Clone, Debug)]
pub struct HuginnComponent {
    pub label: String
}
