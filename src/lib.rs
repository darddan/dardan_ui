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
pub use base::NoParamInit;
pub use base::NoSharedVars;

mod app;
pub use app::UiApp;
//pub type UiApp<T: UiContainer + NoParamInit> = UiApp<T, NoSharedVars>;

pub mod containers;
use containers::UiFreeContainer;

pub type UiApplication = UiApp<UiFreeContainer, NoSharedVars>;
