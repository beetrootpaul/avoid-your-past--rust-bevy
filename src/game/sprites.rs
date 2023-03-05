use bevy::math::vec2;
use bevy::prelude::*;
use image::{DynamicImage, EncodableLayout, RgbaImage};

#[derive(Resource, Default)]
pub struct SpriteSheet {
    // pub texture_atlas_handle: Option<Handle<TextureAtlas>>,
    pub maybe_rgba_image: Option<RgbaImage>,
    pub maybe_bytes: Option<&'static [u8]>,
}

impl SpriteSheet {
    const COLUMNS: usize = 16;
    const ROWS: usize = 4;
    const DEFAULT_SPRITE_W: f32 = 8.;
    const DEFAULT_SPRITE_H: f32 = 8.;

    pub const COIN_FIRST: usize = 0;
    pub const COIN_LAST: usize = 31;
    pub const PLAYER_UP: usize = 34;
    pub const PLAYER_RIGHT: usize = 35;
    pub const PLAYER_DOWN: usize = 36;
    pub const PLAYER_LEFT: usize = 37;
    pub const TRAIL_PARTICLE_5PX: usize = 39;
    pub const TRAIL_PARTICLE_3PX: usize = 40;
    pub const TRAIL_PARTICLE_1PX: usize = 41;
}

pub struct GameSpriteSheetPlugin;

impl Plugin for GameSpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteSheet>()
            .add_startup_system(load_spritesheet);
    }
}

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

fn load_spritesheet(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    /*
    let included_bytes = include_bytes!("../../assets/spritesheet.png");
    let loaded_image = image::load_from_memory(included_bytes).expect("should load image from memory");
    let now_it_is_for_sure_rgba8_image = loaded_image.to_rgba8();
    let rgba8_bytes = now_it_is_for_sure_rgba8_image.as_bytes();
    info!("{} bytes, content: {:?}", rgba8_bytes.len(), rgba8_bytes);
     */

    let included_bytes = include_bytes!("../../assets/spritesheet.png");
    // info!("inlined bytes len: {}", included_bytes.len());
    // info!("inlined bytes: {:?}", included_bytes);
    // println!("{:?}",bytes.len());
    // return bytes.to_vec();

    // let path = "assets/spritesheet.png";
    // let path = "spritesheet.png";
    // let path = "/spritesheet.png";
    // let path = "./spritesheet.png";
    // info!("Path is '{}'", path);

    // let tmp1 = image::RgbaImage::from_raw(128, 32, bytes.to_vec());
    // let tmp2 = tmp1.expect("lol");
    // let tmp3 = DynamicImage::ImageRgba8(tmp2);

    let loaded_image: DynamicImage = image::load_from_memory(included_bytes).expect("should load image from memory");
    // let dyn_img: DynamicImage = image::open(path).expect("LOAD DYN IMG");
    // info!("{:?}", loaded_image.as_bytes().len());
    // info!("{:?}", loaded_image.as_bytes());
    // let rgba8: RgbaImage = dyn_img.to_rgba8();
    // let rgba8_bytes = rgba8.as_bytes();
    // info!("dyn img bytes len: {}", rgba8_bytes.len());
    // info!("dyn img bytes: {:?}", rgba8_bytes);

    // let image_handle: Handle<Image> = asset_server.load("spritesheet.png");
    // let texture_atlas = TextureAtlas::from_grid(
    //     image_handle,
    //     vec2(SpriteSheet::DEFAULT_SPRITE_W, SpriteSheet::DEFAULT_SPRITE_H),
    //     SpriteSheet::COLUMNS,
    //     SpriteSheet::ROWS,
    //     None,
    //     None,
    // );
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    //
    commands.insert_resource(SpriteSheet {
        // texture_atlas_handle: Some(texture_atlas_handle),

        maybe_rgba_image: Some(loaded_image.to_rgba8()),
        // maybe_rgba_image: None,

        // maybe_bytes: Some(bytes),
        maybe_bytes: None,
    });
}
