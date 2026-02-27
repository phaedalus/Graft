use std::fs;
use mlua::{Lua, Function};
use raylib::{ffi::KeyboardKey, RaylibHandle};
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
    fn update(&mut self, dt: f32, rl: &RaylibHandle) {
        let Some(f) = &self.update_fn else { return; };
        let update_fn = f.clone();

        self.lua
            .scope(|scope| {
                let globals = self.lua.globals();

                let is_key_down = scope.create_function(|_, key: String| {
                    let key = string_to_key(&key);
                    Ok(rl.is_key_down(key))
                })?;
                globals.set("is_key_down", is_key_down)?;

                let is_key_pressed = scope.create_function(|_, key: String| {
                    let key = string_to_key(&key);
                    Ok(rl.is_key_pressed(key))
                })?;
                globals.set("is_key_pressed", is_key_pressed)?;

                let mouse_pos = scope.create_function(|_, ()| {
                    let pos = rl.get_mouse_position();
                    Ok((pos.x, pos.y))
                })?;
                globals.set("get_mouse_position", mouse_pos)?;

                update_fn.call::<()>(dt)?;
                Ok(())
            })
            .unwrap();
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

fn string_to_key(name: &str) -> KeyboardKey {
    match name.to_uppercase().as_str() {
        "W" => KeyboardKey::KEY_W,
        "A" => KeyboardKey::KEY_A,
        "S" => KeyboardKey::KEY_S,
        "D" => KeyboardKey::KEY_D,
        "SPACE" => KeyboardKey::KEY_SPACE,
        "ESC" => KeyboardKey::KEY_ESCAPE,
        "UP" => KeyboardKey::KEY_UP,
        "DOWN" => KeyboardKey::KEY_DOWN,
        "LEFT" => KeyboardKey::KEY_LEFT,
        "RIGHT" => KeyboardKey::KEY_LEFT,
        _ => KeyboardKey::KEY_NULL,
    }
}