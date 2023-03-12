use bevy::math::vec2;
use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct SpriteDimensions {
    // TODO: it is confusing how to setup w/h and padding, since w/h are in relation to center,
    //       so it's like we can set w/h to lowest even number and then make paddings of 1px on two edges if needed…
    // TODO: note it down somewhere or make sure it is satisfied in the code itself
    //       (taken from bevy_pixel_camera plugin's README https://github.com/drakmaniso/bevy_pixel_camera#bevy_pixel_camera ):
    //       Note that if either the width or the height of your sprite is not divisible by 2,
    //       you need to change the anchor of the sprite (which is at the center by default),
    //       or it will not be pixel aligned.
    pub width: f32,
    pub height: f32,

    pub padding_left: f32,
    pub padding_right: f32,
    pub padding_top: f32,
    pub padding_bottom: f32,
}

impl Default for SpriteDimensions {
    fn default() -> Self {
        Self {
            width: SpriteSheet::DEFAULT_SPRITE_W,
            height: SpriteSheet::DEFAULT_SPRITE_H,
            padding_left: 0.,
            padding_right: 0.,
            padding_top: 0.,
            padding_bottom: 0.,
        }
    }
}

