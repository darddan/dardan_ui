use crate::{UiAttr, UiElem, UiFixSize, UiParam, UiPos, UiSize, UiSizeVal};

pub struct UiSpace {
    size: UiSize,
    fix_size: UiFixSize,
    needed_size: UiFixSize,
}

impl UiSpace {
    pub fn new() -> Self {
        UiSpace {
            size: UiSize::new(),
            fix_size: UiFixSize::new(),
            needed_size: UiFixSize::new(),
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

    ui_define_size_functions!(Size: size myself {
        myself.needed_size.x = crate::elements::get_needed_val(myself.size.x);
        myself.needed_size.y = crate::elements::get_needed_val(myself.size.y);
    });

    ui_define_size_functions!(FixSize: fix_size);

    ui_define_size_functions!(NeededSize: needed_size);
}
