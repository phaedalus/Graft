use std::fs;
use mlua::{Lua, Function};
use crate::core::{App, Frame};

pub struct LuaRuntime {
    lua: Lua,
    update_fn: Option<Function>,
    draw_fn: Option<Function>,
}

impl LuaRuntime {
    pub fn new(path: &str) -> Self {
        let lua = Lua::new();

        let source = fs::read_to_string(path)
            .expect("Failed to read main.lua");

        lua.load(&source)
            .set_name(path)
            .exec()
            .expect("Failed to execute Lua");

        let globals = lua.globals();

        let update_fn: Option<Function> = globals.get("update").unwrap();
        let draw_fn: Option<Function>   = globals.get("draw").unwrap();

        Self {
            lua,
            update_fn,
            draw_fn,
        }
    }
}

impl App for LuaRuntime {
    fn update(&mut self, dt: f32) {
        if let Some(f) = &self.update_fn {
            let _ = f.call::<()>(dt);
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        if self.draw_fn.is_none() {
            return;
        }

        let draw_fn = self.draw_fn.as_ref().unwrap().clone();

        self.lua
            .scope(|scope| {
                let globals = self.lua.globals();

                let draw_text_fn = scope
                    .create_function_mut(|_, (text, x, y, size): (String, i32, i32, i32)| {
                        frame.text(&text, x, y, size, raylib::prelude::Color::WHITE);
                        Ok(())
                    })?;

                globals.set("draw_text", draw_text_fn)?;

                draw_fn.call::<()>(())?;

                Ok(())
            })
            .unwrap();
    }
}