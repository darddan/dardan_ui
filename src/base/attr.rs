use {UiCol, UiDirection, UiSize};

pub enum UiAttr {
    BackgroundColor(UiCol),
    TextColor(UiCol),
    Size(UiSize),
    Title(String),
    Direction(UiDirection),
}
