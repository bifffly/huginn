use vgtk::*;

pub mod window;
pub mod toolbar;

#[derive(Clone, Debug)]
pub enum Msg {
    BACK,
    NEXT,
    SEARCH,
    EXIT
}

#[derive(Clone, Debug, Default)]
pub struct HuginnWindow {}

#[derive(Clone, Debug, Default)]
pub struct HuginnToolbar {
    pub back_clicked: Callback<()>,
    pub next_clicked: Callback<()>,
    pub search_clicked: Callback<()>
}

