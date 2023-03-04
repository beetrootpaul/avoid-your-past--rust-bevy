use bevy::math::vec3;
use bevy::prelude::*;
// use bevy::sprite::MaterialMesh2dBundle;

use crate::game::collision::HitCircle;
use crate::pico8_color::Pico8Color;
use crate::z_layer::Z_LAYER_DEBUG_HIT_CIRCLES;

pub struct HitCirclesVisualizationPlugin;

#[cfg(debug_assertions)]
impl Plugin for HitCirclesVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HitCirclesVisualizationConfig {
            is_plugin_enabled: false,
        })
            .add_system(update_hit_circle_visualization_visibility);
    }
}

#[derive(Resource)]
pub struct HitCirclesVisualizationConfig {
    pub is_plugin_enabled: bool,
}

#[cfg(debug_assertions)]
pub fn create_hit_circle_visualization(
    hit_circle: &HitCircle,
    z_layer_of_parent: f32,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    (
        HitCircleVisualization,
        // MaterialMesh2dBundle {
        //     mesh: meshes.add(shape::Circle::new(hit_circle.r).into()).into(),
        //     material: materials.add(ColorMaterial::from(Pico8Color::Red.as_bevy_color())),
        //     This transform is relative to the parent entity
        // transform: Transform::from_translation(vec3(
        //     hit_circle.offset.x,
        //     hit_circle.offset.y,
        //     Z_LAYER_DEBUG_HIT_CIRCLES - z_layer_of_parent,
        // )),
        // ..default()
        // },
    )
}

#[derive(Component)]
struct HitCircleVisualization;

fn update_hit_circle_visualization_visibility(
    // mut query: Query<&mut Visibility, With<HitCircleVisualization>>,
    hit_circles_visualization_config: Res<HitCirclesVisualizationConfig>,
) {
    // for mut visibility in query.iter_mut() {
    //     visibility.is_visible = hit_circles_visualization_config.is_plugin_enabled;
    // }
}
