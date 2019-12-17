use crate::{UiAttr, UiCol, UiElem, UiFixSize, UiParam, UiPos, UiSize, UiSizeVal};

pub struct UiFill {
    size: UiSize,
    fix_size: UiFixSize,
    needed_size: UiFixSize,
    background_color: ::sdl2::pixels::Color,
}

impl Default for UiFill {
    fn default() -> Self {
        UiFill {
            size: UiSize::default(),
            fix_size: UiFixSize::default(),
            needed_size: UiFixSize::default(),
            background_color: UiCol::new(255, 255, 255, 255).sdl2(),
        }
    }
}

impl UiFill {
    pub fn set_background_color(&mut self, color: UiCol) {
        self.background_color = color.sdl2();
    }
}

impl UiElem for UiFill {
    fn draw(&self, canvas: &mut ::sdl2::render::Canvas<::sdl2::video::Window>, cv_pos: &UiPos) {
        crate::util::draw_rect(canvas, cv_pos, &self.fix_size, self.background_color);
    }

    fn set_attribute(&mut self, attr: UiAttr) {
        match attr {
            UiAttr::Size(val) => self.set_size(val),
            UiAttr::FixSize(val) => self.set_fix_size(val),
            UiAttr::BackgroundColor(val) => self.set_background_color(val),
            _ => (),
        }
    }

    fn set_value(&mut self, value: UiParam) {
        if let UiParam::Attr(attr) = value {
            self.set_attribute(attr);
        }
    }

    ui_define_size_functions!(Size: size myself {
        myself.needed_size.x = crate::elements::get_needed_val(myself.size.x);
        myself.needed_size.y = crate::elements::get_needed_val(myself.size.y);
    });

    ui_define_size_functions!(FixSize: fix_size);

    ui_define_size_functions!(NeededSize: needed_size);
}
