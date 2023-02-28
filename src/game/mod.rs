use bevy::prelude::*;

pub use coin::create_coin_spawn_systems;
pub use coin::Coin;
pub use input::KeyboardControlsPlugin;
pub use player::create_player_move_systems;
pub use player::create_player_spawn_systems;
pub use player::PlayerMovement;
pub use sprites::SpriteDimensions;
pub use sprites::SpriteSheetPlugin;

use crate::game::game_area::{spawn_game_area, GAME_AREA_H, GAME_AREA_W};
use crate::game::gui::TOPBAR_H;
use crate::game::logic::create_collect_coins_systems;
#[cfg(debug_assertions)]
use crate::game::sprites_debug::SpritesBoundariesPlugin;
use crate::pico8_color::Pico8Color;
use crate::pixel_art_support::{FixedFpsBevyAppExtension, FixedFpsPlugin, PixelArtCameraPlugin};

mod coin;
mod collision;
mod game_area;
mod gui;
mod input;
mod logic;
mod player;
mod sprites;
pub mod sprites_debug;

pub const GAME_TITLE: &str = "Avoid Your Past";

pub const VIEWPORT_W: f32 = GAME_AREA_W;
pub const VIEWPORT_H: f32 = TOPBAR_H + GAME_AREA_H;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KeyboardControlsPlugin);

        app.add_plugin(SpriteSheetPlugin);

        app.add_plugin(PixelArtCameraPlugin);

        #[cfg(debug_assertions)]
        app.add_plugin(SpritesBoundariesPlugin);

        app.insert_resource(ClearColor(Pico8Color::Black.as_bevy_color()));

        app.add_startup_system(spawn_game_area);

        // TODO: prevent FixedFpsPlugin from being added twice in different places in the app
        app.add_plugin(FixedFpsPlugin);
        #[cfg(debug_assertions)]
        app.log_fixed_fps_measurements();
        app.add_fixed_fps_stage(vec![create_player_move_systems()]);
        app.add_fixed_fps_stage(vec![create_collect_coins_systems()]);
        app.add_fixed_fps_stage(vec![
            create_player_spawn_systems(),
            create_coin_spawn_systems(),
        ]);
    }
}
