use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::game::gui::TOPBAR_H;
use crate::pico8_color::Pico8Color;
use crate::z_layer::Z_LAYER_GAME_AREA;

pub const GAME_AREA_W: f32 = 128.;
pub const GAME_AREA_H: f32 = 112.;

pub struct GameAreaPlugin;

impl Plugin for GameAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_game_area);
    }
}

pub fn spawn_game_area(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Pico8Color::DarkBlue.as_bevy_color(),
            custom_size: Some(vec2(GAME_AREA_W, GAME_AREA_H)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        transform: Transform::from_xyz(
            -GAME_AREA_W / 2.,
            GAME_AREA_H / 2. - TOPBAR_H / 2.,
            Z_LAYER_GAME_AREA,
        ),
        ..default()
    });
}
