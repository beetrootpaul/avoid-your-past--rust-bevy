use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    InGame,
    DebugResumeFor1Frame,
    DebugPause,
}

impl GameState {
    // pub fn should_game_update(current_state: Res<CurrentState<GameState>>) -> bool {
    //     matches!(
    //         *current_state,
    //         CurrentState(GameState::InGame) | CurrentState(GameState::DebugResumeFor1Frame)
    //     )
    // }
}

pub fn create_system_update_game_state() -> SystemSet {
    SystemSet::new()
    // .with_system(update_game_state)
}

// fn update_game_state(current_state: Res<CurrentState<GameState>>, mut commands: Commands) {
//     if let CurrentState(GameState::DebugResumeFor1Frame) = *current_state {
//         commands.insert_resource(NextState(GameState::DebugPause))
//     }
// }
