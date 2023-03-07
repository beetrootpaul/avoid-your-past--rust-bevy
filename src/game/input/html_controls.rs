use bevy::prelude::*;
use wasm_bindgen::prelude::*;

use crate::game::PlayerMovement;

#[wasm_bindgen]
extern "C" {
    fn should_turn_left() -> bool;
    fn should_turn_right() -> bool;
    fn should_turn_up() -> bool;
    fn should_turn_down() -> bool;
}

pub struct GameHtmlControlsPlugin;

impl Plugin for GameHtmlControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_html_input);
    }
}

// TODO: handle a case of multiple arrows pressed at once
fn handle_html_input(mut player_movement_query: Query<&mut PlayerMovement>) {
    if should_turn_left() {
        for mut player_movement in player_movement_query.iter_mut() {
            *player_movement = PlayerMovement::Left;
        }
    }
    if should_turn_right() {
        for mut player_movement in player_movement_query.iter_mut() {
            *player_movement = PlayerMovement::Right;
        }
    }
    if should_turn_up() {
        for mut player_movement in player_movement_query.iter_mut() {
            *player_movement = PlayerMovement::Up;
        }
    }
    if should_turn_down() {
        for mut player_movement in player_movement_query.iter_mut() {
            *player_movement = PlayerMovement::Down;
        }
    }
}
