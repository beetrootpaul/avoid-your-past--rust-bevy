use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use iyes_loopless::prelude::ConditionSet;

use crate::game::collision::HitCircle;
#[cfg(debug_assertions)]
use crate::game::collision_debug::create_hit_circle_visualization;
use crate::game::game_area::{GAME_AREA_H, GAME_AREA_W};
use crate::game::game_state::GameState;
use crate::game::gui::TOPBAR_H;
use crate::game::sprites::{SpriteDimensions, SpriteSheet};
use crate::game::trail::TrailOrigin;
use crate::pixel_art_support::FixedFpsTime;
use crate::z_layer::Z_LAYER_SPRITES_PLAYER;

pub fn create_systems_player_spawn() -> SystemSet {
    ConditionSet::new()
        .run_if(GameState::should_game_update)
        .run_if(there_is_no_player)
        .with_system(spawn_player)
        .into()
}

pub fn create_systems_player_move() -> SystemSet {
    ConditionSet::new()
        .run_if(GameState::should_game_update)
        .with_system(move_player)
        .with_system(update_player_sprite)
        .into()
}

fn update_player_sprite(mut query: Query<(&PlayerMovement, &mut TextureAtlasSprite)>) {
    for (player_movement, mut sprite) in query.iter_mut() {
        sprite.index = get_sprite_index_for_movement(player_movement);
    }
}

fn get_sprite_index_for_movement(movement: &PlayerMovement) -> usize {
    match *movement {
        PlayerMovement::Up => SpriteSheet::PLAYER_UP,
        PlayerMovement::Right => SpriteSheet::PLAYER_RIGHT,
        PlayerMovement::Down => SpriteSheet::PLAYER_DOWN,
        PlayerMovement::Left => SpriteSheet::PLAYER_LEFT,
    }
}
