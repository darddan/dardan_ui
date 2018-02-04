use Canvas;
use Window;
use Color;


pub enum UiAttribute {
    BackgroundColor(Color),
    TextColor(Color),
}

pub struct UiPair<T> {
    pub x: T,
    pub y: T,
}

pub trait UiElement {
    fn init(&self);
    fn draw(&self, canvas: &mut Canvas<Window>, pos: &UiPair<i32>);
    fn get_parent(&self) -> Option<Box<UiContainer>>;
    fn set_attribute(&mut self, attribute: UiAttribute);
    fn set_attributes(&mut self, attributes: Vec<UiAttribute>) {
        for attribute in attributes {
            self.set_attribute(attribute);
        }
    }
    fn draw_ui(&self, canvas: &mut Canvas<Window>, pos: &UiPair<i32>) {
        self.draw(canvas, pos);
    }

    fn draw_me(&self, canvas: &mut Canvas<Window>, pos: &UiPair<i32>) {
        self.draw_ui(canvas, pos);
    }
    fn set_size(&mut self, size: &UiPair<u32>);
}

pub trait NoParamInit {
    fn new() -> Self;
}

pub struct NoSharedVars {}

impl NoParamInit for NoSharedVars {
    fn new() -> Self {
        NoSharedVars {}
    }
}

pub trait UiContainer: UiElement {
    fn get_children(&self) -> &Vec<Box<UiElement>>;
    fn draw_ui(&self, canvas: &mut Canvas<Window>, pos: &UiPair<i32>) {
        self.draw(canvas, pos);
        for i in self.get_children().iter() {
            i.draw_ui(canvas, pos);
        }
    }
}
