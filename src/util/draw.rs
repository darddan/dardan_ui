use {UiFixSize, UiPos};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[inline(always)]
pub fn draw_rect(canvas: &mut Canvas<Window>, cv_pos: &UiPos, size: &UiFixSize, color: Color) {
    canvas.set_draw_color(color);
    let _ = canvas.fill_rect(Rect::new(cv_pos.x, cv_pos.y, size.x, size.y));
}
