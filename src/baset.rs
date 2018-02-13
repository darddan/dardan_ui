use {UiCol, UiElem, UiCell, UiSize};


pub struct UiPair<T> {
    pub x: T,
    pub y: T,
}

impl Clone for UiPair<u32> {
    fn clone(&self) -> Self {
        UiPair {
            x: self.x,
            y: self.y,
        }
    }
}

impl UiPair<i32> {
    pub fn new_i32() -> Self {
        UiPair { x: 0, y: 0 }
    }
}
impl UiPair<u32> {
    pub fn new_u32() -> Self {
        UiPair { x: 0, y: 0 }
    }
}

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

