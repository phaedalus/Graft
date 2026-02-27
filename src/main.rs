mod core;
mod lua;

use core::runtime::{Runtime, RuntimeConfig};
use lua::runtime::LuaRuntime;

fn main() {
    let app = LuaRuntime::new("main.lua");

    let config = RuntimeConfig {
        width: 1280,
        height: 720,
        title: "Graft Game".to_string(),
        target_fps: 60,
        resizable: false,
        fullscreen: false,
        borderless: false,
    };

    let runtime = Runtime::new(config, app);
    runtime.run();
}