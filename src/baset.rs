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
