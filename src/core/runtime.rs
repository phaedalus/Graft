use raylib::prelude::*;
use super::{App, Frame};

pub struct RuntimeConfig {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub target_fps: u32,
    pub resizable: bool,
    pub fullscreen: bool,
    pub borderless: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            title: "Graft".to_string(),
            target_fps: 60,
            resizable: false,
            fullscreen: false,
            borderless: false,
        }
    }
}

pub struct Runtime<A: App> {
    config: RuntimeConfig,
    app: A,
}

impl<A: App> Runtime<A> {
    pub fn new(config: RuntimeConfig, app: A) -> Self {
        Self { config, app }
    }

    pub fn run(mut self) {
        let title = self.config.title.clone();

        let mut binding = raylib::init();
        let mut builder = binding
            .size(self.config.width, self.config.height)
            .title(&title);

        if self.config.resizable {
            builder = builder.resizable();
        }

        if self.config.borderless {
            builder = builder.undecorated()
        }

        let (mut rl, thread) = builder.build();

        if self.config.fullscreen {
            rl.toggle_fullscreen();
        }

        rl.set_target_fps(self.config.target_fps);

        let fixed_time_step = 1.0 / self.config.target_fps as f32;
        let mut accumulator: f32 = 0.0;

        while !rl.window_should_close() {
            let frame_time = rl.get_frame_time();
            accumulator += frame_time;

            while accumulator >= fixed_time_step {
                self.app.update(fixed_time_step, &rl);
                accumulator -= fixed_time_step;
            }

            let d = rl.begin_drawing(&thread);
            let mut frame = Frame::new(d);

            frame.clear(Color::BLACK);
            self.app.draw(&mut frame);
        }
    }
}