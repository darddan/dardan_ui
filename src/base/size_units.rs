#[derive(Copy, Clone)]
pub enum UiSizeVal {
    Max,
    Min,
    Px(u32),
    Rel(u8),
}

#[derive(Clone)]
pub struct UiSize {
    pub x: UiSizeVal,
    pub y: UiSizeVal,
}

impl UiSize {
    pub fn new() -> Self {
        UiSize {
            x: UiSizeVal::Max,
            y: UiSizeVal::Max,
        }
    }
}

#[derive(Clone)]
pub struct UiFixSize {
    pub x: u32,
    pub y: u32,
}

impl UiFixSize {
    pub fn new() -> Self {
        UiFixSize { x: 0, y: 0 }
    }
}

#[derive(Clone)]
pub struct UiPos {
    pub x: i32,
    pub y: i32,
}

impl UiPos {
    pub fn new() -> Self {
        UiPos { x: 0, y: 0 }
    }
}
