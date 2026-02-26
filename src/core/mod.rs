pub mod runtime;
pub mod frame;
pub use frame::Frame;

pub trait App {
    fn update(&mut self, dt: f32);
    fn draw(&mut self, frame: &mut Frame); 
}