---@meta

-------------------------------------------------------
-- Engine Lifecycle Hooks
-------------------------------------------------------

--- Called every fixed timestep.
--- @param dt number
function update(dt) end

--- Called once per frame.
function draw() end

-------------------------------------------------------
-- Rendering API
-------------------------------------------------------

--- Draw text to screen.
--- @param text string
--- @param x number
--- @param y number
--- @param size number
function draw_text(text, x, y, size) end

-------------------------------------------------------
-- Input API (Available during update)
-------------------------------------------------------

--- Returns true while the key is held.
--- @param key string
--- @return boolean
function is_key_down(key) end

--- Returns true on the frame the key was pressed.
--- @param key string
--- @return boolean
function is_key_pressed(key) end

--- Returns mouse position
--- @return number x
--- @return number y
function get_mouse_position() end