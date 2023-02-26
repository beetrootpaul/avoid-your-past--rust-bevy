use std::ops::Add;

use bevy::math::{vec3, Vec3Swizzles};
use bevy::prelude::{App, Plugin, Query, ResMut, TextureAtlasSprite, Transform};

use crate::constants::{
    SPRITE_SHEET_SPRITE_H, SPRITE_SHEET_SPRITE_W, Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
};

pub struct SpritesBoundariesPlugin;

impl Plugin for SpritesBoundariesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin::default())
            .add_system(draw_debug_sprite_boundaries);
    }
}

fn draw_debug_sprite_boundaries(
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    query: Query<(&Transform, &TextureAtlasSprite)>,
) {
    for (transform, _sprite) in query.iter() {
        // TODO: can we do it in a shorter way?
        lines.line(
            transform.translation.xyz().add(vec3(
                SPRITE_SHEET_SPRITE_W / 2.,
                SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            transform.translation.xyz().add(vec3(
                -SPRITE_SHEET_SPRITE_W / 2.,
                SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            0.,
        );
        lines.line(
            transform.translation.xyz().add(vec3(
                -SPRITE_SHEET_SPRITE_W / 2.,
                SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            transform.translation.xyz().add(vec3(
                -SPRITE_SHEET_SPRITE_W / 2.,
                -SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            0.,
        );
        lines.line(
            transform.translation.xyz().add(vec3(
                -SPRITE_SHEET_SPRITE_W / 2.,
                -SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            transform.translation.xyz().add(vec3(
                SPRITE_SHEET_SPRITE_W / 2.,
                -SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            0.,
        );
        lines.line(
            transform.translation.xyz().add(vec3(
                SPRITE_SHEET_SPRITE_W / 2.,
                -SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            transform.translation.xyz().add(vec3(
                SPRITE_SHEET_SPRITE_W / 2.,
                SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            0.,
        );
    }
}
