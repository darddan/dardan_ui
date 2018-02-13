use sdl2::pixels::Color;

pub struct UiCol {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl UiCol {
    pub fn sdl2(&self) -> Color {
        Color::RGBA(self.r, self.g, self.b, self.a)
    }
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        UiCol { r, g, b, a }
    }
    // Color names taken from:
    // https://graf1x.com/wp-content/uploads/2017/06/list-of-colors-and-color-names.jpg
    pub fn red() -> Self {
        UiCol::new(0xD3, 0x00, 0x00, 0xFF)
    }
    pub fn green() -> Self {
        UiCol::new(0x3B, 0xB1, 0x43, 0xFF)
    }
    pub fn blue() -> Self {
        UiCol::new(0x00, 0x18, 0xF9, 0xFF)
    }
    pub fn salmon() -> Self {
        UiCol::new(0xFA, 0x80, 0x72, 0xFF)
    }
}