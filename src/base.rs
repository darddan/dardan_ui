use std::rc::Rc;
use std::cell::RefCell;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub struct UiCol {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl UiCol {
    pub fn sdl2(&self) -> Color {
        Color::RGBA(self.r, self.g, self.b, self.a)
    }
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        UiCol { r, g, b, a }
    }
    // Color names taken from:
    // https://graf1x.com/wp-content/uploads/2017/06/list-of-colors-and-color-names.jpg
    pub fn red() -> Self {
        UiCol::new(0xD3, 0x00, 0x00, 0xFF)
    }
    pub fn green() -> Self {
        UiCol::new(0x3B, 0xB1, 0x43, 0xFF)
    }
    pub fn blue() -> Self {
        UiCol::new(0x00, 0x18, 0xF9, 0xFF)
    }
    pub fn salmon() -> Self {
        UiCol::new(0xFA, 0x80, 0x72, 0xFF)
    }
}

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
    Size(UiPair<u32>),
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
    Child(Rc<RefCell<UiElem>>),
    RelChild(UiRelSize, Rc<RefCell<UiElem>>),
}

pub trait UiElem {
    fn draw(&self, canvas: &mut Canvas<Window>, cv_pos: &UiPair<i32>);
    // This function will be used to implement redraw
    // This also affects that the same instance of an object cant be used in two different UiApps
    fn get_parent(&self) -> Option<Rc<RefCell<UiElem>>>;
    fn set_parent(&mut self, parent: Rc<RefCell<UiElem>>);
    fn set_attribute(&mut self, attr: UiAttr);
    fn set_attributes(&mut self, attr_vec: Vec<UiAttr>) {
        for attr in attr_vec {
            self.set_attribute(attr);
        }
    }
    fn set_value(&mut self, value: UiParam);
    fn set_values(&mut self, values: Vec<UiParam>) {
        for value in values {
            self.set_value(value);
        }
    }
    fn get_size(&self) -> UiPair<u32>;
    fn set_size(&mut self, size: UiPair<u32>);
    // TODO : Add redraw function
    // TODO : Add draw_partly function (which will be used by UiScrollCombo)
}
