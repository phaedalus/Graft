use raylib::prelude::*;

pub struct Frame<'a> {
    d: RaylibDrawHandle<'a>,
}

impl<'a> Frame<'a> {
    pub fn new(d: RaylibDrawHandle<'a>) -> Self {
        Self { d }
    }

    pub fn clear(&mut self, color: Color) {
        self.d.clear_background(color);
    }

    pub fn text(&mut self, text: &str, x: i32, y: i32, size: i32, color: Color) {
        self.d.draw_text(text, x, y, size, color);
    }
}