use sdl2::render::Canvas;
use sdl2::video::Window;

use {UiAttr, UiElem, UiFixSize, UiPair, UiParam, UiSize};

pub struct UiEmptyItem {
    size: UiSize,
    fix_size: UiFixSize,
}

impl UiEmptyItem {
    pub fn new() -> Self {
        UiEmptyItem {
            size: UiSize::new(),
            fix_size: UiFixSize::new(),
        }
    }
}

impl UiElem for UiEmptyItem {
    fn draw(&self, _canvas: &mut Canvas<Window>, _cv_pos: &UiPair<i32>) {
        // Do Nothing
    }

    fn set_attribute(&mut self, attr: UiAttr) {
        match attr {
            UiAttr::Size(val) => self.size = val,
            _ => (),
        }
    }

    fn set_value(&mut self, value: UiParam) {
        match value {
            UiParam::Attr(attr) => self.set_attribute(attr),
            _ => (),
        }
    }

    fn get_size(&self) -> UiSize {
        self.size.clone()
    }

    fn set_size(&mut self, size: UiSize) {
        self.size = size;
    }

    fn get_fix_size(&self) -> UiFixSize {
        self.fix_size.clone()
    }

    fn set_fix_size(&mut self, size: UiFixSize) {
        self.fix_size = size;
    }
}
