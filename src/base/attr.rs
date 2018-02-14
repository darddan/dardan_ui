use {UiCol, UiDirection, UiFixSize, UiSize};

pub enum UiAttr {
    BackgroundColor(UiCol),
    TextColor(UiCol),
    Size(UiSize),
    FixSize(UiFixSize),
    Title(String),
    Direction(UiDirection),
}
