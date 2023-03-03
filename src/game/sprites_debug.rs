use std::ops::Add;

use bevy::ecs::schedule::ShouldRun;
use bevy::math::{vec3, Vec3Swizzles};
use bevy::prelude::*;

use crate::game::sprites::SpriteDimensions;
use crate::z_layer::Z_LAYER_DEBUG_SPRITE_BOUNDARIES;

#[cfg(debug_assertions)]
pub struct SpritesBoundariesPlugin;

#[cfg(debug_assertions)]
impl Plugin for SpritesBoundariesPlugin {
    // Uses https://crates.io/crates/bevy_prototype_debug_lines
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin::default())
            .insert_resource(SpritesBoundariesConfig {
                is_plugin_enabled: false,
            })
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(is_plugin_enabled)
                    .with_system(draw_debug_sprite_boundaries),
            );
    }
}

#[derive(Resource)]
pub struct SpritesBoundariesConfig {
    pub is_plugin_enabled: bool,
}

fn is_plugin_enabled(config: Res<SpritesBoundariesConfig>) -> ShouldRun {
    match config.is_plugin_enabled {
        true => ShouldRun::Yes,
        false => ShouldRun::No,
    }
}

#[cfg(debug_assertions)]
fn draw_debug_sprite_boundaries(
    query: Query<(&Transform, &SpriteDimensions)>,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
) {
    let corners_1_clockwise = [(1., 1.), (1., -1.), (-1., -1.), (-1., 1.)];
    let mut corners_2_clockwise = corners_1_clockwise;
    corners_2_clockwise.rotate_left(1);
    let corner_pairs_clockwise = corners_1_clockwise.iter().zip(corners_2_clockwise.iter());

    for (transform, sprite_dimensions) in query.iter() {
        // Sprite inner boundary
        for ((x_sign_1, y_sign_1), (x_sign_2, y_sign_2)) in corner_pairs_clockwise.clone() {
            let p_w_1 = if *x_sign_1 > 0. {
                sprite_dimensions.padding_right
            } else {
                sprite_dimensions.padding_left
            };
            let p_h_1 = if *y_sign_1 > 0. {
                sprite_dimensions.padding_top
            } else {
                sprite_dimensions.padding_bottom
            };
            let p_w_2 = if *x_sign_2 > 0. {
                sprite_dimensions.padding_right
            } else {
                sprite_dimensions.padding_left
            };
            let p_h_2 = if *y_sign_2 > 0. {
                sprite_dimensions.padding_top
            } else {
                sprite_dimensions.padding_bottom
            };
            lines.line(
                transform.translation.xyz().add(vec3(
                    x_sign_1 * (sprite_dimensions.width / 2. - p_w_1),
                    y_sign_1 * (sprite_dimensions.height / 2. - p_h_1),
                    Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
                )),
                transform.translation.xyz().add(vec3(
                    x_sign_2 * (sprite_dimensions.width / 2. - p_w_2),
                    y_sign_2 * (sprite_dimensions.height / 2. - p_h_2),
                    Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
                )),
                0.,
            );
        }

        // Sprite outer boundary
        for ((x_sign_1, y_sign_1), (x_sign_2, y_sign_2)) in corner_pairs_clockwise.clone() {
            lines.line(
                transform.translation.xyz().add(vec3(
                    x_sign_1 * sprite_dimensions.width / 2.,
                    y_sign_1 * sprite_dimensions.height / 2.,
                    Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
                )),
                transform.translation.xyz().add(vec3(
                    x_sign_2 * sprite_dimensions.width / 2.,
                    y_sign_2 * sprite_dimensions.height / 2.,
                    Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
                )),
                0.,
            );
        }
    }
}
