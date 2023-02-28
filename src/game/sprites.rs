use bevy::math::vec2;
use bevy::prelude::*;

// TODO: is is possible to bind these into a single parent object?
const SPRITE_SHEET_SPRITE_W: f32 = 8.;
const SPRITE_SHEET_SPRITE_H: f32 = 8.;
const SPRITE_SHEET_COLUMNS: usize = 16;
const SPRITE_SHEET_ROWS: usize = 3;

#[derive(Resource, Default)]
pub struct SpriteSheet {
    pub texture_atlas_handle: Option<Handle<TextureAtlas>>,
}

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteSheet>()
            .add_startup_system(ss_load_spritesheet);
    }
}

#[derive(Component, Copy, Clone)]
pub struct SpriteDimensions {
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
            width: SPRITE_SHEET_SPRITE_W,
            height: SPRITE_SHEET_SPRITE_H,
            padding_left: 0.,
            padding_right: 0.,
            padding_top: 0.,
            padding_bottom: 0.,
        }
    }
}

fn ss_load_spritesheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image_handle: Handle<Image> = asset_server.load("spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        image_handle,
        vec2(SPRITE_SHEET_SPRITE_W, SPRITE_SHEET_SPRITE_H),
        SPRITE_SHEET_COLUMNS,
        SPRITE_SHEET_ROWS,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(SpriteSheet {
        texture_atlas_handle: Some(texture_atlas_handle),
    });
}
