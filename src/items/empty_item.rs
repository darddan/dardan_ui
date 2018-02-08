use std::rc::Rc;
use std::cell::RefCell;

use sdl2::render::Canvas;
use sdl2::video::Window;

use {UiAttr, UiPair, UiSetParam, UiUnit};

pub struct UiEmptyItem {
    size: UiPair<u32>,
    parent: Option<Rc<RefCell<UiUnit>>>,
}

impl UiEmptyItem {
    pub fn new() -> Self {
        UiEmptyItem {
            size: UiPair::new_u32(),
            parent: None,
        }
    }
}

impl UiUnit for UiEmptyItem {
    fn draw(&self, _canvas: &mut Canvas<Window>, _cv_pos: &UiPair<i32>) {
        // Do Nothing
    }

    fn get_parent(&self) -> Option<Rc<RefCell<UiUnit>>> {
        self.parent.clone()
    }

    fn set_parent(&mut self, parent: Rc<RefCell<UiUnit>>) {
        self.parent = Some(parent);
    }

    fn set_attribute(&mut self, attr: UiAttr) {
        match attr {
            UiAttr::Size(val) => self.size = val,
            _ => (),
        }
    }

    fn set_value(&mut self, value: UiSetParam) {
        match value {
            UiSetParam::Attr(attr) => self.set_attribute(attr),
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
