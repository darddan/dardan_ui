use sdl2::render::Canvas;
use sdl2::video::Window;

use {UiAttr, UiFixSize, UiParam, UiPos, UiSize, UiSizeVal};

pub trait UiElem {
    fn draw(&self, canvas: &mut Canvas<Window>, cv_pos: &UiPos);
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
    fn set_x(&mut self, x: UiSizeVal);
    fn set_y(&mut self, y: UiSizeVal);
    fn get_size(&self) -> UiSize;
    fn get_x(&self) -> UiSizeVal;
    fn get_y(&self) -> UiSizeVal;
    fn set_fix_size(&mut self, size: UiFixSize);
    fn set_fix_x(&mut self, x: u32);
    fn set_fix_y(&mut self, y: u32);
    fn get_fix_size(&self) -> UiFixSize;
    fn get_fix_x(&self) -> u32;
    fn get_fix_y(&self) -> u32;
}
