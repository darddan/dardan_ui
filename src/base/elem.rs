use sdl2::render::Canvas;
use sdl2::video::Window;

use {UiAttr, UiFixSize, UiPair, UiParam, UiSize};

pub trait UiElem {
    fn draw(&self, canvas: &mut Canvas<Window>, cv_pos: &UiPair<i32>);
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
    fn set_size(&mut self, size: UiSize);
    fn get_size(&self) -> UiSize;
    fn set_fix_size(&mut self, size: UiFixSize);
    fn get_fix_size(&self) -> UiFixSize;
}
