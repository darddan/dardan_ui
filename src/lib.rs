extern crate sdl2;
pub use sdl2::pixels::Color;
pub use sdl2::render::Canvas;
pub use sdl2::video::Window;
pub use sdl2::rect::Rect;

mod base;
pub use base::UiElement;
pub use base::UiContainer;
pub use base::UiAttribute;
pub use base::UiPair;

mod app;
pub use app::UiApp;

pub mod containers;
