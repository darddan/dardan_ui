use std::time::Duration;

use {Canvas, Color, UiElement, UiPair, Window};

use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::render::BlendMode;

pub struct UiApp {
    pub context: Sdl,
    pub subsystem: VideoSubsystem,
    pub title: String,
    pub main_element: Option<Box<UiElement>>,
}

impl UiApp {
    pub fn new(title: String) -> Self {
        let sdl_context = ::sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        UiApp {
            context: sdl_context,
            subsystem: video_subsystem,
            title: title,
            main_element: None,
        }
    }

    pub fn set_main_element(&mut self, elem: Box<UiElement>) {
        self.main_element = Some(elem);
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, position: &UiPair<i32>) {
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();
        let ui_option = self.main_element.take();
        let ui_elem = ui_option.unwrap();
        ui_elem.draw_ui(canvas, position);
        self.main_element = Some(ui_elem);
    }

    fn set_size(&mut self, size: &UiPair<u32>) {
        let ui_option = self.main_element.take();
        let mut ui_elem = ui_option.unwrap();
        ui_elem.set_size(size);
        self.main_element = Some(ui_elem);
    }

    pub fn run(&mut self) {
        let window = self.subsystem
            .window(&self.title, 500, 500)
            .vulkan()
            .resizable()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_blend_mode(BlendMode::Blend);

        let mut event_pump = self.context.event_pump().unwrap();

        self.handle_loop(&mut event_pump, &mut canvas);
    }

    fn handle_loop(&mut self, event_pump: &mut EventPump, mut canvas: &mut Canvas<Window>) {
        let position = UiPair { x: 0, y: 0 };

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::Window {
                        win_event: WindowEvent::Resized(x_val, y_val),
                        ..
                    } => {
                        self.set_size(&UiPair {
                            x: x_val as u32,
                            y: y_val as u32,
                        });
                        self.draw(&mut canvas, &position);
                    }
                    _ => {}
                }
            }

            canvas.present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
