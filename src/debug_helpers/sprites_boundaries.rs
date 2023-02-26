use std::ops::Add;

use bevy::math::{vec3, Vec3Swizzles};
use bevy::prelude::{App, Plugin, Query, ResMut, TextureAtlasSprite, Transform, With};

use crate::constants::{
    SPRITE_SHEET_SPRITE_H, SPRITE_SHEET_SPRITE_W, Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
};

pub struct SpritesBoundariesPlugin;

impl Plugin for SpritesBoundariesPlugin {
    // Uses https://crates.io/crates/bevy_prototype_debug_lines
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin::default())
            .add_system(draw_debug_sprite_boundaries);
    }
}

fn draw_debug_sprite_boundaries(
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    query: Query<&Transform, With<TextureAtlasSprite>>,
) {
    let corners_1_clockwise = [(1., 1.), (1., -1.), (-1., -1.), (-1., 1.)];
    let corners_2_clockwise = [(1., -1.), (-1., -1.), (-1., 1.), (1., 1.)];
    for transform in query.iter() {
        for ((x_sign_1, y_sign_1), (x_sign_2, y_sign_2)) in
            corners_1_clockwise.iter().zip(corners_2_clockwise.iter())
        {
            lines.line(
                transform.translation.xyz().add(vec3(
                    x_sign_1 * SPRITE_SHEET_SPRITE_W / 2.,
                    y_sign_1 * SPRITE_SHEET_SPRITE_H / 2.,
                    Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
                )),
                transform.translation.xyz().add(vec3(
                    x_sign_2 * SPRITE_SHEET_SPRITE_W / 2.,
                    y_sign_2 * SPRITE_SHEET_SPRITE_H / 2.,
                    Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
                )),
                0.,
            );
        }
    }
}
