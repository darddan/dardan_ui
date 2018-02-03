use Color;
use Canvas;
use Window;
use Rect;
use UiElement;
use UiContainer;
use UiAttribute;
use UiPair;

pub struct UiFreeContainer {
    background: Color,
    child: Vec<Box<UiElement>>,
    size: UiPair<u32>,
}

impl UiFreeContainer {
    pub fn new() -> Self {
        UiFreeContainer {
            background: Color {
                r: 122,
                g: 0,
                b: 0,
                a: 125,
            },
            child: vec![],
            size: UiPair { x: 200, y: 200 },
        }
    }
}

impl UiElement for UiFreeContainer {
    fn init(&self) {}

    fn get_parent(&self) -> Option<Box<UiContainer>> {
        None
    }

    fn set_attribute(&mut self, attribute: UiAttribute) {
        match attribute {
            UiAttribute::BackgroundColor(x) => self.background = x.unwrap(),
            _ => {}
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, pos: &UiPair<i32>) {
        canvas.set_draw_color(self.background);

        let the_rectangle = Rect::new(pos.x, pos.y, self.size.x, self.size.y);
        canvas.fill_rect(the_rectangle).ok();
    }
    fn set_size(&mut self, size: &UiPair<u32>) {
        self.size.x = size.x;
        self.size.y = size.y;
    }
}

impl UiContainer for UiFreeContainer {
    fn get_children(&self) -> &Vec<Box<UiElement>> {
        &self.child
    }
}
