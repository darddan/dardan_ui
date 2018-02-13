use sdl2::render::Canvas;
use sdl2::video::Window;

use {UiAttr, UiPair, UiParam, UiElem};

pub struct UiEmptyItem {
    size: UiPair<u32>,
}

impl UiEmptyItem {
    pub fn new() -> Self {
        UiEmptyItem {
            size: UiPair::new_u32(),
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

    fn get_size(&self) -> UiPair<u32> {
        self.size.clone()
    }

    fn set_size(&mut self, size: UiPair<u32>) {
        self.size = size;
    }
}
