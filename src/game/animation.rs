use bevy::prelude::{Component, Query, SystemSet, TextureAtlasSprite};
use iyes_loopless::prelude::IntoConditionalSystem;
use crate::game::GameState;

// TODO: assert first <= last

#[derive(Component)]
pub struct AnimationFrames {
    pub first: usize,
    pub last: usize,
}

pub fn create_systems_animate_sprite() -> SystemSet {
    SystemSet::new().with_system(animate_sprites.run_in_state(GameState::InGame))
}

fn animate_sprites(mut query: Query<(&AnimationFrames, &mut TextureAtlasSprite)>) {
    for (animation_frames, mut sprite) in query.iter_mut() {
        sprite.index = if sprite.index < animation_frames.last {
            sprite.index + 1
        } else {
            animation_frames.first
        }
    }
}
