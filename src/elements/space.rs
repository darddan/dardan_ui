use {UiAttr, UiElem, UiFixSize, UiParam, UiPos, UiSize, UiSizeVal};

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

    define_size_functions!(Size: size);
    
    define_size_functions!(FixSize: fix_size);
}
