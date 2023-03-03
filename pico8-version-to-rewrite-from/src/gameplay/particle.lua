-- -- -- -- -- -- -- --
-- gameplay/particle --
-- -- -- -- -- -- -- --

function new_particle(params)
    local x = params.x
    local y = params.y
    local color = params.color

    local r_max = 2
    local ttl_max = 28
    local ttl = ttl_max

    local p = {}

    --

    function p.age()
        ttl = max(0, ttl - 1)
    end

    --

    function p.should_disappear()
        return ttl <= 0
    end

    --

    function p.draw()
        local r = flr((ttl / ttl_max) * (r_max + 0.9))
        circfill(x, y, r, color)
    end

    --

    return p
end