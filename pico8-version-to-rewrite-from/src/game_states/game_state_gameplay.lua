-- -- -- -- -- -- -- -- -- -- -- -- --
-- game_states/game_state_gameplay  --
-- -- -- -- -- -- -- -- -- -- -- -- --

function new_game_state_gameplay(params)
    local mode = params.mode
    local topbar = params.topbar
    local score = params.score
    local level = params.level
    local player = params.player

    local memories = new_memories {
        player = player,
    }
    local player_trail = new_trail {
        origin = player,
        color = u.colors.dark_green,
    }

    local function on_back_to_regular_mode()
        audio.enable_music_layers { true, false, false }
    end

    local function on_coin_collision()
        if mode.is_no_coins() then
            return
        end

        audio.play_sfx(a.sfx_coin)
        score.add(10)
        if not mode.is_no_memories() then
            memories.add_memory()
        end
        level.remove_coin()
        level.spawn_items()
    end

    local function on_droplet_no_coins_collision()
        audio.enable_music_layers { true, false, true }
        score.add(3)
        mode.start_no_coins()
        level.remove_droplet_no_coins()
    end

    local function on_droplet_no_memories_collision()
        audio.enable_music_layers { true, true, false }
        score.add(1)
        mode.start_no_memories()
        level.remove_droplet_no_memories()
    end

    audio.enable_music_layers { true, false, false }

    local gs = {}

    --

    function gs.update()
        if btnp(u.buttons.l) then
            player.direct_left()
        elseif btnp(u.buttons.r) then
            player.direct_right()
        elseif btnp(u.buttons.u) then
            player.direct_up()
        elseif btnp(u.buttons.d) then
            player.direct_down()
        end

        mode.update {
            on_back_to_regular_mode = on_back_to_regular_mode
        }

        level.check_collisions {
            on_coin = on_coin_collision,
            on_droplet_no_coins = on_droplet_no_coins_collision,
            on_droplet_no_memories = on_droplet_no_memories_collision,
        }

        level.animate()

        player_trail.update()
        player.move()

        memories.move()

        if not mode.is_no_memories() then
            if memories.has_player_collided_with_memory() then
                return new_game_state_over {
                    score = score,
                    level = level,
                    player = player,
                }
            end
        end

        return gs
    end

    --

    function gs.draw()
        level.draw_bg()
        level.draw_items()

        player_trail.draw()
        player.draw()

        if not mode.is_no_memories() then
            memories.draw()
        end

        topbar.draw()
    end

    --

    return gs
end