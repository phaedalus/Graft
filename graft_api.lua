---@meta

-- Engine lifecycle hooks.

--- Called every fixed timestep.
--- @param dt number
function update(dt) end

--- Called once per frame.
function draw() end

-- Rendering API

--- Draw text to screen.
--- @param text string
--- @param x number
--- @param y number
--- @param size number
function draw_text(text, x, y, size) end