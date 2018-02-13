use std::rc::Rc;
use std::cell::RefCell;
use sdl2::render::Canvas;
use sdl2::video::Window;

use {UiPair, UiAttr, UiParam};

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