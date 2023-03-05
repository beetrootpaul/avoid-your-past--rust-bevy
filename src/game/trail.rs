use bevy::prelude::*;

use crate::game::game_state::GameState;
use crate::game::sprites::{SpriteDimensions, SpriteSheet};
use crate::pixel_art_support::FixedFpsTime;
use crate::z_layer::Z_LAYER_SPRITES_TRAILS;

pub fn create_systems_trail_particles_spawn() -> SystemSet {
    SystemSet::new()
    // ConditionSet::new()
    //     .run_if(GameState::should_game_update)
    //     .with_system(spawn_particles)
    //     .into()
}

pub fn create_systems_trail_particles_age() -> SystemSet {
    SystemSet::new()
    // ConditionSet::new()
    //     .run_if(GameState::should_game_update)
    //     .with_system(age_particles)
    //     .into()
}

#[derive(Component)]
pub struct TrailOrigin {
    spawn_particle_timer: Timer,
}

impl TrailOrigin {
    pub fn with_seconds_between_particles(seconds: f32) -> Self {
        Self {
            spawn_particle_timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}

#[derive(Bundle)]
struct TrailParticleBundle {
    trail_particle: TrailParticle,
    // sprite_sheet_bundle: SpriteSheetBundle,
    sprite_dimensions: SpriteDimensions,
}

#[derive(Component)]
struct TrailParticle {
    ttl_frames: i32,
}

impl Default for TrailParticle {
    fn default() -> Self {
        Self { ttl_frames: 26 }
    }
}

fn spawn_particles(
    mut commands: Commands,
    mut query: Query<(&mut TrailOrigin, &Transform)>,
    fixed_fps_time: Res<FixedFpsTime>,
    sprite_sheet: Res<SpriteSheet>,
) {
    for (mut trail_origin, origin_transform) in query.iter_mut() {
        trail_origin
            .spawn_particle_timer
            .tick(fixed_fps_time.duration);
        if trail_origin.spawn_particle_timer.just_finished() {
            let mut particle_transform = *origin_transform;
            particle_transform.translation.z = Z_LAYER_SPRITES_TRAILS;
            commands.spawn(TrailParticleBundle {
                trail_particle: TrailParticle::default(),
                // sprite_sheet_bundle: SpriteSheetBundle {
                //     transform: particle_transform,
                //     texture_atlas: sprite_sheet.texture_atlas_handle.clone().unwrap(),
                //     sprite: TextureAtlasSprite {
                //         index: SpriteSheet::TRAIL_PARTICLE_5PX,
                //         anchor: Anchor::Center,
                //         ..default()
                //     },
                //     ..default()
                // },
                sprite_dimensions: SpriteDimensions {
                    width: 6.,
                    height: 6.,
                    padding_right: 1.,
                    padding_bottom: 1.,
                    ..default()
                },
            });
        }
    }
}

// TODO: write tests for cases of TTL 1, 0, and in general for off-by-1 errors
fn age_particles(
    mut commands: Commands,
    // mut query: Query<(Entity, &mut TrailParticle, Option<&mut TextureAtlasSprite>)>,
) {
    // for (particle_entity, mut trail_particle, maybe_sprite) in query.iter_mut() {
    //     trail_particle.ttl_frames -= 1;
    //     if trail_particle.ttl_frames <= 0 {
    //         commands.entity(particle_entity).despawn_recursive();
    //     } else if let Some(mut sprite) = maybe_sprite {
    //         sprite.index = match trail_particle.ttl_frames {
    //             18.. => SpriteSheet::TRAIL_PARTICLE_5PX,
    //             8.. => SpriteSheet::TRAIL_PARTICLE_3PX,
    //             _ => SpriteSheet::TRAIL_PARTICLE_1PX,
    //         }
    //     }
    // }
}
