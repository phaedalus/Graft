mod core;
mod lua;

use core::runtime::Runtime;
use lua::runtime::LuaRuntime;

fn main() {
    let lua_app = LuaRuntime::new("main.lua");

    let runtime = Runtime::new(60, lua_app);
    runtime.run();
}