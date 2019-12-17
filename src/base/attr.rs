use crate::{UiCol, UiFixSize, UiSize};

pub enum UiAttr {
    BackgroundColor(UiCol),
    TextColor(UiCol),
    Size(UiSize),
    FixSize(UiFixSize),
    Title(String),
}
