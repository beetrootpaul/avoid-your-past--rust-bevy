use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;

use crate::game::{VIEWPORT_H, VIEWPORT_W};
use crate::pico8_color::Pico8Color;

pub struct PixelArtRenderingPlugin;

impl Plugin for PixelArtRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Pico8Color::Black.as_bevy_color()));
        app.add_plugin(bevy_pixel_camera::PixelCameraPlugin);
        app.add_startup_system(setup_pixel_art_rendering);
    }
}

fn setup_pixel_art_rendering(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let viewport_size = Extent3d {
        width: VIEWPORT_W as u32,
        height: VIEWPORT_H as u32,
        ..default()
    };

    // Copied from https://github.com/bevyengine/bevy/blob/main/examples/3d/render_to_texture.rs
    let mut image_to_render_on = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: viewport_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };
    image_to_render_on.resize(viewport_size);
    let handle_to_image_to_render_on = images.add(image_to_render_on);

    let mut image_based_camera = Camera2dBundle::default();
    image_based_camera.camera.target = RenderTarget::Image(handle_to_image_to_render_on.clone());
    commands.spawn((image_based_camera, ));

    let output_render_layer = RenderLayers::layer(1);

    commands.spawn((
        output_render_layer,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(vec2(VIEWPORT_W, VIEWPORT_H)),
                ..default()
            },
            texture: handle_to_image_to_render_on.clone(),
            ..default()
        },
    ));

    commands.spawn((
        output_render_layer,
        bevy_pixel_camera::PixelCameraBundle::from_resolution(VIEWPORT_W as i32, VIEWPORT_H as i32),
    ));
}
