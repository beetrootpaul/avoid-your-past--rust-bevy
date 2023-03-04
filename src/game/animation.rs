use bevy::prelude::{Component, Query, SystemSet};
use iyes_loopless::prelude::ConditionSet;

use crate::game::game_state::GameState;

// TODO: assert first <= last

#[derive(Component)]
pub struct AnimationFrames {
    pub first: usize,
    pub last: usize,
}

pub fn create_systems_animate_sprite() -> SystemSet {
    ConditionSet::new()
        .run_if(GameState::should_game_update)
        // .with_system(animate_sprites)
        .into()
}

// fn animate_sprites(mut quer/y: Query<(&AnimationFrames, &mut TextureAtlasSprite)>) {
//     for (animation_frames, mut sprite) in query.iter_mut() {
//         sprite.index = if sprite.index < animation_frames.last {
//             sprite.index + 1
//         } else {
//             animation_frames.first
//         }
//     }
// }
