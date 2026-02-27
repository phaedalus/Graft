pub mod runtime;
pub mod frame;

pub use frame::Frame;

use raylib::prelude::RaylibHandle;

pub trait App {
    fn update(&mut self, dt: f32, rl: &RaylibHandle);
    fn draw(&mut self, frame: &mut Frame); 
}