use {UiAttr, UiCol, UiElem, UiFixSize, UiParam, UiPos, UiSize};

pub struct UiFill {
    size: UiSize,
    fix_size: UiFixSize,
    background_color: ::sdl2::pixels::Color,
}

impl UiFill {
    pub fn new() -> Self {
        UiFill {
            size: UiSize::new(),
            fix_size: UiFixSize::new(),
            background_color: UiCol::new(255, 255, 255, 255).sdl2(),
        }
    }
    pub fn set_background_color(&mut self, color: UiCol) {
        self.background_color = color.sdl2();
    }
}

impl UiElem for UiFill {
    fn draw(&self, canvas: &mut ::sdl2::render::Canvas<::sdl2::video::Window>, cv_pos: &UiPos) {
        ::util::draw_rect(canvas, cv_pos, &self.fix_size, self.background_color);
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
