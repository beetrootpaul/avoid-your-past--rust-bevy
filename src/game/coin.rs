use bevy::math::vec3;
use bevy::prelude::*;
// use bevy::sprite::Anchor;

use crate::game::animation::AnimationFrames;
use crate::game::collision::HitCircle;
#[cfg(debug_assertions)]
use crate::game::collision_debug::create_hit_circle_visualization;
use crate::game::game_state::GameState;
use crate::game::gui::TOPBAR_H;
use crate::game::sprites::{SpriteDimensions, SpriteSheet};
use crate::z_layer::Z_LAYER_SPRITES_COINS;

#[derive(Component)]
pub struct Coin;

#[derive(Bundle)]
struct CoinBundle {
    coin: Coin,
    // sprite_sheet_bundle: SpriteSheetBundle,
    animation_frames: AnimationFrames,
    sprite_dimensions: SpriteDimensions,
    hit_circle: HitCircle,
}

pub fn create_systems_coin_spawn() -> SystemSet {
    SystemSet::new()
    // ConditionSet::new()
    //     .run_if(GameState::should_game_update)
    //     .run_if(there_is_no_coin)
    //     .with_system(spawn_coin)
    //     .into()
}

fn there_is_no_coin(query: Query<&Coin>) -> bool {
    query.iter().count() == 0
}

fn spawn_coin(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    // meshes: ResMut<Assets<Mesh>>,
    // materials: ResMut<Assets<ColorMaterial>>,
) {
    // let mut rng = rand::thread_rng();

    let animation_frames = AnimationFrames {
        first: SpriteSheet::COIN_FIRST,
        last: SpriteSheet::COIN_LAST,
    };
    let hit_circle = HitCircle {
        r: 3.7,
        offset: vec3(0., 0., 0.),
    };
    let mut parent_command = commands.spawn(CoinBundle {
        coin: Coin,
        // sprite_sheet_bundle: SpriteSheetBundle {
        // TODO: reorganize game area position calculations
        // TODO: add helpers for translating from window-centered coors to game area coords
        // TODO: TEMPORARY COORDS
        // transform: Transform::from_xyz(
        //     rng.gen_range(-40.0..40.0),
        //     -TOPBAR_H / 2. + rng.gen_range(-40.0..40.0),
        //     Z_LAYER_SPRITES_COINS,
        // ),
        // texture_atlas: sprite_sheet.texture_atlas_handle.clone().unwrap(),
        // sprite: TextureAtlasSprite {
        //     index: animation_frames.first,
        //     anchor: Anchor::Center,
        //     ..default()
        // },
        // ..default()
        // },
        animation_frames,
        sprite_dimensions: SpriteDimensions {
            width: 6.,
            height: 6.,
            ..default()
        },
        hit_circle: hit_circle.clone(),
    });

    #[cfg(debug_assertions)]
    parent_command.with_children(|parent| {
        parent.spawn(create_hit_circle_visualization(
            &hit_circle,
            Z_LAYER_SPRITES_COINS,
            // meshes,
            // materials,
        ));
    });
}
