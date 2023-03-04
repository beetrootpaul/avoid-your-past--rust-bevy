use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

pub use coin::create_systems_coin_spawn;
pub use coin::Coin;
pub use input::GameKeyboardControlsPlugin;
pub use player::create_systems_player_move;
pub use player::create_systems_player_spawn;
pub use player::PlayerMovement;
pub use sprites::GameSpriteSheetPlugin;

use crate::game::animation::create_systems_animate_sprite;
use crate::game::audio::GameAudioPlugin;
use crate::game::collision_debug::HitCirclesVisualizationPlugin;
use crate::game::game_area::{spawn_game_area, GAME_AREA_H, GAME_AREA_W};
use crate::game::game_state::{create_system_update_game_state, GameState};
use crate::game::gui::TOPBAR_H;
use crate::game::logic::create_systems_collect_coins;
#[cfg(debug_assertions)]
use crate::game::sprites_debug::SpritesBoundariesPlugin;
use crate::game::trail::{
    create_systems_trail_particles_age, create_systems_trail_particles_spawn,
};
use crate::pico8_color::Pico8Color;
use crate::pixel_art_support::{FixedFpsBevyAppExtension, FixedFpsPlugin, PixelArtCameraPlugin};
pub use sprites::SpriteSheet;

mod animation;
mod audio;
mod coin;
mod collision;
mod collision_debug;
mod game_area;
mod game_state;
mod gui;
mod input;
mod logic;
mod player;
mod sprites;
mod sprites_debug;
mod trail;

pub const GAME_TITLE: &str = "Avoid Your Past";
pub const VIEWPORT_W: f32 = GAME_AREA_W;
pub const VIEWPORT_H: f32 = TOPBAR_H + GAME_AREA_H;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::InGame);

        // app.add_plugin(GameKeyboardControlsPlugin);

        // TODO: add some nice assertions for whether plugin was added or not, because right now error is very cryptic
        app.add_plugin(GameSpriteSheetPlugin);
        // app.add_plugin(GameAudioPlugin);

        // app.add_plugin(PixelArtCameraPlugin);

        // #[cfg(debug_assertions)]
        // app.add_plugin(SpritesBoundariesPlugin);
        // #[cfg(debug_assertions)]
        // app.add_plugin(HitCirclesVisualizationPlugin);

        // app.insert_resource(ClearColor(Pico8Color::Black.as_bevy_color()));

        // app.add_startup_system(spawn_game_area);

        // TODO: prevent FixedFpsPlugin from being added twice in different places in the app
        app.add_plugin(FixedFpsPlugin);
        #[cfg(debug_assertions)]
        app.log_fixed_fps_measurements();
        // app.add_fixed_fps_stage(vec![create_systems_trail_particles_age()]);
        // app.add_fixed_fps_stage(vec![create_systems_player_move()]);
        // app.add_fixed_fps_stage(vec![create_systems_collect_coins()]);
        // app.add_fixed_fps_stage(vec![create_systems_animate_sprite()]);
        app.add_fixed_fps_stage(vec![
            create_systems_player_spawn(),
            // create_systems_coin_spawn(),
        ]);
        // app.add_fixed_fps_stage(vec![create_systems_trail_particles_spawn()]);
        // app.add_fixed_fps_stage(vec![create_system_update_game_state()]);
    }
}
