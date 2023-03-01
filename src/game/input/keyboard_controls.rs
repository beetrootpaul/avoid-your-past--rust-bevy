use bevy::prelude::*;
use crate::game::PlayerMovement;

pub struct GameKeyboardControlsPlugin;

impl Plugin for GameKeyboardControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_keyboard_input);
    }
}

// TODO: handle a case of multiple arrows pressed at once
fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut PlayerMovement>,
) {
    if keyboard_input.just_pressed(KeyCode::Left) {
        for mut player_movement in query.iter_mut() {
            *player_movement = PlayerMovement::Left;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        for mut player_movement in query.iter_mut() {
            *player_movement = PlayerMovement::Right;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        for mut player_movement in query.iter_mut() {
            *player_movement = PlayerMovement::Up;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        for mut player_movement in query.iter_mut() {
            *player_movement = PlayerMovement::Down;
        }
    }
}
