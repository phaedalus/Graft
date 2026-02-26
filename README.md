# 🌿 Graft

Graft is a minimal Rust runtime foundation used across Labyrinthin titles.

It is designed to be forked per game and exposes Lua for gameplay scripting.

Graft is not a monolithic engine.

Is is a stable core + a rendering surface + a scripting bridge.

---

## Principles

- Fork per game
- Core remains minimal
- No feature creep
- Lua for gameplay logic
- Rust for performance
- Systems modular and replaceable

---

## Current Capabilities

- Fixed timestep runtime loop
- Raylib-backend renderer
- Frame-scoped drawing surface
- Lua `update(dt)` and `draw()` lifecycle hooks.

---

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