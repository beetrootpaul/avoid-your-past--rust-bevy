-- -- -- --
-- main  --
-- -- -- --

--__debug__ = true

local gs

function _init()
    gs = new_game_state_splash()
end

function _update()
    gs = gs.update()
end

function _draw()
    cls()
    camera(a.camera_x, a.camera_y)
    gs.draw()
end
