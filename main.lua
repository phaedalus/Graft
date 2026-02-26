local t = 0

function update(dt)
    t = t + dt
    if t > 1 then
        print("Lua tick")
        t = 0
    end
end

function draw()
    draw_text("Hello Graft", 100, 100, 30)
end