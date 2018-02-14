use UiCell;

use {UiElem, UiFixSize, UiPos};
use elements::UiFill;

use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, Canvas};
use sdl2::video::Window;

pub struct UiApp {
    name: String,
    size: UiFixSize,
    main_element: UiCell<UiElem>,
    context: Sdl,
    subsystem: VideoSubsystem,
}

impl UiApp {
    pub fn new() -> Self {
        let sdl_context = ::sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        UiApp {
            context: sdl_context,
            subsystem: video_subsystem,
            name: String::from("Ui Application"),
            size: UiFixSize { x: 400, y: 400 },
            main_element: ::new_ui_cell(UiFill::new()),
        }
    }

    pub fn set_title(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_main_element(&mut self, main_element: UiCell<UiElem>) {
        self.main_element = main_element;
    }

    pub fn run(&mut self) {
        let window = self.subsystem
            .window(&self.name, 500, 500)
            .resizable()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_blend_mode(BlendMode::Blend);

        let mut event_pump = self.context.event_pump().unwrap();

        self.handle_loop(&mut event_pump, &mut canvas);
    }

    #[inline(always)]
    fn handle_loop(&mut self, event_pump: &mut EventPump, mut canvas: &mut Canvas<Window>) {
        let position = UiPos::new();

        self.set_size();
        self.draw(&mut canvas, &position);

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
                        self.size = UiFixSize {
                            x: x_val as u32,
                            y: y_val as u32,
                        };
                        self.set_size();
                        self.draw(&mut canvas, &position);
                    }
                    _ => {}
                }
            }

            canvas.present();

            ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, position: &UiPos) {
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();
        let elem = self.main_element.write().unwrap();
        elem.draw(canvas, position);
    }

    pub fn set_size(&mut self) {
        self.main_element.write().unwrap().set_fix_size(UiFixSize {
            x: self.size.x,
            y: self.size.y,
        });
    }
}
