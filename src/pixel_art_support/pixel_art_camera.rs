use bevy::prelude::{App, Plugin};

use crate::pico8_color::Pico8Color;

pub struct PixelArtCameraPlugin;

impl Plugin for PixelArtCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_pixel_camera::PixelCameraPlugin);
        app.add_plugin(bevy_pixel_camera::PixelBorderPlugin {
            color: Pico8Color::Black.as_bevy_color(),
        });
    }
}
