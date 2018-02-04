use std::time::Duration;

use {Canvas, Color, NoParamInit, UiContainer, UiPair, Window};

use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::render::BlendMode;

pub struct UiApp<T: UiContainer + NoParamInit, U: NoParamInit> {
    pub context: Sdl,
    pub subsystem: VideoSubsystem,
    pub title: String,
    pub size: UiPair<u32>,
    main_element: T,
    shared_variables: U,
}

impl<T: UiContainer + NoParamInit, U: NoParamInit> UiApp<T, U> {
    pub fn new() -> Self {
        let sdl_context = ::sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        UiApp {
            context: sdl_context,
            subsystem: video_subsystem,
            title: String::from("Dardan"),
            size: UiPair { x: 200, y: 400 },
            main_element: T::new(),
            shared_variables: U::new(),
        }
    }

    pub fn use_main_element(&mut self) -> &mut T {
        &mut self.main_element
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

    fn draw(&mut self, canvas: &mut Canvas<Window>, position: &UiPair<i32>) {
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();
        self.main_element.draw_me(canvas, position);
        //let ui_option = self.main_element.take();
        //let ui_elem = ui_option.unwrap();
        //ui_elem.draw_ui(canvas, position);
        //self.main_element = Some(ui_elem);
    }

    fn set_size(&mut self, size: &UiPair<u32>) {
        self.main_element.set_size(size);
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
