use {UiAttr, UiElem, UiFixSize, UiParam, UiPos, UiSize};

pub struct UiSpace {
    size: UiSize,
    fix_size: UiFixSize,
}

impl UiSpace {
    pub fn new() -> Self {
        UiSpace {
            size: UiSize::new(),
            fix_size: UiFixSize::new(),
        }
    }
}

impl UiElem for UiSpace {
    fn draw(&self, _canvas: &mut ::sdl2::render::Canvas<::sdl2::video::Window>, _cv_pos: &UiPos) {
        // Do Nothing
    }

    fn set_attribute(&mut self, attr: UiAttr) {
        match attr {
            UiAttr::Size(val) => self.size = val,
            UiAttr::FixSize(val) => self.fix_size = val,
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
