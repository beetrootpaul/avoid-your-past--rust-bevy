-- -- -- -- -- -- --
-- gameplay/score --
-- -- -- -- -- -- --

function new_score()
    local value = 0

    local s = {}

    --

    function s.value()
        return value
    end

    --

    function s.add(points)
        value = value + points
    end

    --

    return s
end