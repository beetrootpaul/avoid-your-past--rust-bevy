use bevy::prelude::*;

use crate::game::{VIEWPORT_H, VIEWPORT_W};
use crate::pico8_color::Pico8Color;

pub struct PixelArtCameraPlugin;

impl Plugin for PixelArtCameraPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugin(bevy_pixel_camera::PixelCameraPlugin);

        // Color of the background area around the constrained view of the camera
        // app.add_plugin(bevy_pixel_camera::PixelBorderPlugin {
        //     color: Pico8Color::Black.as_bevy_color(),
        // });

        // app.add_startup_system(|mut commands: Commands| {
        //     commands.spawn(bevy_pixel_camera::PixelCameraBundle::from_resolution(
        //         VIEWPORT_W as i32,
        //         VIEWPORT_H as i32,
        //     ));
        // });
    }
}
