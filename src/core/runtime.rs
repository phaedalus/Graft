use raylib::prelude::*;
use super::App;

pub struct Runtime<A: App> {
    target_fps: u32,
    fixed_time_step: f32,
    app: A,
}

impl<A: App> Runtime<A> {
    pub fn new(target_fps: u32, app: A) -> Self {
        Self {
            target_fps,
            fixed_time_step: 1.0 / target_fps as f32,
            app,
        }
    }

    pub fn run(mut self) {
        let (mut rl, thread) = raylib::init()
            .size(1280, 720)
            .title("Graft")
            .build();

        rl.set_target_fps(self.target_fps);

        let mut accumulator: f32 = 0.0;

        while !rl.window_should_close() {
            let frame_time = rl.get_frame_time();
            accumulator += frame_time;

            while accumulator >= self.fixed_time_step {
                self.app.update(self.fixed_time_step);
                accumulator -= self.fixed_time_step;
            }

            let d = rl.begin_drawing(&thread);
            let mut frame = crate::core::Frame::new(d);

            frame.clear(Color::BLACK);

            self.app.draw(&mut frame);
        }
    }
}