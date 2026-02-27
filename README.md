# 🌿 Graft

Graft is a minimal Rust runtime foundation used across Labyrinthin titles.

It is designed to be forked per game and exposes Lua for gameplay scripting.

Graft is not a monolithic engine.

It is a stable core + a rendering surface + a scripting bridge.

---

## Principles

- Fork per game
- Core remains minimal
- No feature creep
- Lua for gameplay logic
- Rust for performance
- Systems modular and replaceable

---

## Current Capabilities (v0.0.2)

- Fixed timestep runtime loop
- Raylib-backend window and renderer
- Frame-scoped drawing surface
- Boot-time window configuration (size, title, fullscreen, resizable, borderless)
- Lua `update(dt)` and `draw()` lifecycle hooks.
- Basic input pollin in `update(dt)`:
    - `is_key_down(key)`
    - `is_key_pressed(key)`
    - `get_mouse_position()`

---

## Architecture Overview

Runtime phases:

1. Input (available during `update`)
2. Fixed-step update
3. Frame-scoped rendering

Rendering is isolated from input.
Lua bindings are scoped per phase.
No unsafe lifetime hacks are used.

## Intended Use

Graft is meant to be forked.

1. Fork the repository.
2. Rename the project
3. Modify `main.lua`
4. Build your game

Each title owns its version permanently.

---

Graft is a foundation, not a product.

## Liscense

MIT