use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::game::collision::HitCircle;
use crate::pico8_color::Pico8Color;

use crate::z_layer::Z_LAYER_DEBUG_HIT_CIRCLES;

#[cfg(debug_assertions)]
pub fn create_hit_circle_debug(
    hit_circle: &HitCircle,
    z_layer_of_parent: f32,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(hit_circle.r * 3.).into()).into(),
        material: materials.add(ColorMaterial::from(Pico8Color::Red.as_bevy_color())),
        // This transform is relative to the parent entity
        transform: Transform::from_translation(vec3(
            hit_circle.offset.x,
            hit_circle.offset.y,
            Z_LAYER_DEBUG_HIT_CIRCLES - z_layer_of_parent,
        )),
        ..default()
    }
}
