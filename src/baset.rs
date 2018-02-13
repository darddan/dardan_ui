use {UiCell, UiCol, UiElem, UiSize};

pub enum UiAttr {
    BackgroundColor(UiCol),
    TextColor(UiCol),
    Size(UiSize),
    Title(String),
    Direction(UiDirection),
}

pub enum UiRelSize {
    Inherit,
    Max,
    Rel(u8), // value between 0 and 100 (Percent)
    Px(u32),
}

#[derive(PartialEq)]
pub enum UiDirection {
    Horizontal,
    Vertical,
}

pub enum UiParam {
    Attr(UiAttr),
    Child(UiCell<UiElem>),
    RelChild(UiRelSize, UiCell<UiElem>),
}
