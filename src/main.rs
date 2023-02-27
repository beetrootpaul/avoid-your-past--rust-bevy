use std::ops::Add;

use bevy::ecs::schedule::ShouldRun;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use rand::Rng;

use crate::constants::{
    SPRITE_SHEET_SPRITE_H, SPRITE_SHEET_SPRITE_W, Z_LAYER_DEBUG_HIT_CIRCLES, Z_LAYER_GAME_AREA,
    Z_LAYER_SPRITES_COINS, Z_LAYER_SPRITES_PLAYER,
};
#[cfg(debug_assertions)]
use crate::debug_helpers::{PrintFpsPlugin, SpritesBoundariesPlugin};
use crate::pico8_color::Pico8Color;
use crate::pixel_art_support::{FixedFpsBevyAppExtension, FixedFpsPlugin, PixelArtCameraPlugin};

mod constants;
#[cfg(debug_assertions)]
mod debug_helpers;
mod pico8_color;
mod pixel_art_support;

// TODO: copy README content from the original repo, add some screenshots
// TODO: non-CC license which allows to use, but not commercially

// TODO: gamepad resource 1: https://github.com/bevyengine/bevy/blob/latest/examples/input/gamepad_input.rs
// TODO: gamepad resource 2: https://github.com/bevyengine/bevy/blob/latest/examples/input/gamepad_input_events.rs
// TODO: gamepad resource 2: https://github.com/bevyengine/bevy/blob/latest/examples/tools/gamepad_viewer.rs

// TODO: touch input resource 1: https://github.com/bevyengine/bevy/blob/latest/examples/input/touch_input.rs
// TODO: touch input resource 2: https://github.com/bevyengine/bevy/blob/latest/examples/input/touch_input_events.rs

// TODO: UI resource 1: https://github.com/bevyengine/bevy/blob/latest/examples/ui/button.rs
// TODO: UI resource 2: https://github.com/bevyengine/bevy/blob/latest/examples/ui/text.rs
// TODO: UI resource 3: https://github.com/bevyengine/bevy/blob/latest/examples/ui/text_debug.rs
// TODO: UI resource 4: https://github.com/bevyengine/bevy/blob/latest/examples/ui/ui.rs
// TODO: UI resource 5: https://github.com/bevyengine/bevy/blob/latest/examples/ui/ui_scaling.rs
// TODO: UI resource 6: https://github.com/bevyengine/bevy/blob/latest/examples/2d/text2d.rs
// TODO: UI resource 7: https://github.com/bevyengine/bevy/blob/latest/examples/ui/font_atlas_debug.rs
// TODO: UI resource 8: https://github.com/bevyengine/bevy/blob/latest/examples/games/game_menu.rs
// TODO: UI resource 9: https://github.com/bevyengine/bevy/blob/latest/examples/2d/2d_shapes.rs

// TODO: audio resource 1: https://github.com/bevyengine/bevy/blob/latest/examples/audio/audio.rs
// TODO: audio resource 2: https://github.com/bevyengine/bevy/blob/latest/examples/audio/audio_control.rs

// TODO: fixed FPS 1 https://github.com/bevyengine/bevy/blob/latest/examples/ecs/fixed_timestep.rs
// TODO: fixed FPS 2 https://bevy-cheatbook.github.io/features/fixed-timestep.html

// TODO: logs https://github.com/bevyengine/bevy/blob/latest/examples/app/logs.rs

// TODO: game states 1 https://github.com/bevyengine/bevy/blob/latest/examples/ecs/state.rs
// TODO: game states 2 https://github.com/IyesGames/iyes_loopless

// TODO: components on sparse sets: https://bevy-cheatbook.github.io/patterns/component-storage.html

// TODO add printing out all resource types (maybe use a proper logging method instead of printing one) https://bevy-cheatbook.github.io/cookbook/print-resources.html

const GAME_TITLE: &str = "Avoid Your Past";

const TOPBAR_H: f32 = 16.;
const GAME_AREA_W: f32 = 128.;
const GAME_AREA_H: f32 = 112.;

const VIEWPORT_W: f32 = GAME_AREA_W;
const VIEWPORT_H: f32 = TOPBAR_H + GAME_AREA_H;

// TODO: app.add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(0.5)).with_system(…))

// TODO consider system sets or system labels for making sure input is handled first, then update, then draw, then removal of dead entities
//      - res 1 : https://bevy-cheatbook.github.io/programming/system-order.html
//      - res 2 : https://bevy-cheatbook.github.io/programming/system-sets.html

// TODO game states https://bevy-cheatbook.github.io/programming/states.html

#[derive(Resource, Default)]
struct SpriteSheet {
    texture_atlas_handle: Option<Handle<TextureAtlas>>,
}

#[derive(Component, Clone)]
struct HitCircle {
    r: f32,
    offset: Vec3,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: GAME_TITLE.to_string(),
                    ..default()
                },
                ..default()
            })
            // Prevent blurring of scaled up pixel art sprites
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                // Watch for changes in assets and hot-reload them without need to run the app again
                watch_for_changes: true,
                ..default()
            }),
    )
    .add_plugin(PixelArtCameraPlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(PrintFpsPlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(SpritesBoundariesPlugin);

    // Get rid of edges of neighbour sprites visible around the given sprite from the sprite sheet
    app.insert_resource(Msaa { samples: 1 })
        // Draw a solid background color
        .insert_resource(ClearColor(Pico8Color::Black.as_bevy_color()));

    app.init_resource::<SpriteSheet>()
        .add_startup_system(load_spritesheet)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_game_area)
        // TODO: will it affect HTML embedded game?
        .add_system(bevy::window::close_on_esc)
        .add_system(handle_keyboard_input);

    app.add_plugin(FixedFpsPlugin);
    #[cfg(debug_assertions)]
    app.log_fixed_fps_measurements();
    app.add_fixed_fps_stage(vec![
        SystemSet::new()
            .with_run_criteria(run_if_there_is_no_player)
            .with_system(spawn_player),
        SystemSet::new()
            .with_run_criteria(run_if_there_is_no_coin)
            .with_system(spawn_coin),
    ]);
    app.add_fixed_fps_stage(vec![SystemSet::new()
        .with_system(fixed_update_player)
        .with_system(fixed_update_player_sprite)]);
    app.add_fixed_fps_stage(vec![SystemSet::new().with_system(coin_pickup)]);

    app.run();
}

#[derive(Component)]
enum ControlledDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component, Default, Copy, Clone)]
struct SpritePadding {
    #[allow(dead_code)]
    left: i32,
    #[allow(dead_code)]
    right: i32,
    #[allow(dead_code)]
    top: i32,
    #[allow(dead_code)]
    bottom: i32,
}

fn spawn_camera(mut commands: Commands) {
    // TODO: proper conversion from f32 to i32?
    commands.spawn(bevy_pixel_camera::PixelCameraBundle::from_resolution(
        VIEWPORT_W as i32,
        VIEWPORT_H as i32,
    ));
}

fn spawn_game_area(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Pico8Color::DarkBlue.as_bevy_color(),
            custom_size: Some(vec2(GAME_AREA_W, GAME_AREA_H)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        transform: Transform::from_xyz(
            -GAME_AREA_W / 2.,
            GAME_AREA_H / 2. - TOPBAR_H / 2.,
            Z_LAYER_GAME_AREA,
        ),
        ..default()
    });
}

// TODO: note it down somewhere or make sure it is satisfied in the code itself
//       (taken from bevy_pixel_camera plugin's README https://github.com/drakmaniso/bevy_pixel_camera#bevy_pixel_camera ):
//       Note that if either the width or the height of your sprite is not divisible by 2,
//       you need to change the anchor of the sprite (which is at the center by default),
//       or it will not be pixel aligned.

fn load_spritesheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_sheet_handle: Handle<Image> = asset_server.load("spritesheet.png");
    let sprite_sheet_texture_atlas = TextureAtlas::from_grid(
        sprite_sheet_handle,
        vec2(SPRITE_SHEET_SPRITE_W, SPRITE_SHEET_SPRITE_H),
        16,
        3,
        None,
        None,
    );
    let sprite_sheet_texture_atlas_handle = texture_atlases.add(sprite_sheet_texture_atlas);

    commands.insert_resource(SpriteSheet {
        texture_atlas_handle: Some(sprite_sheet_texture_atlas_handle),
    });
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Coin;

fn run_if_there_is_no_player(query: Query<&Player>) -> ShouldRun {
    if query.iter().count() > 0 {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

fn run_if_there_is_no_coin(query: Query<&Coin>) -> ShouldRun {
    if query.iter().count() > 0 {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

fn coin_pickup(
    mut commands: Commands,
    players_query: Query<(&Transform, &HitCircle), With<Player>>,
    coins_query: Query<(Entity, &Transform, &HitCircle), With<Coin>>,
) {
    for (player_transform, player_hit_circle) in players_query.iter() {
        for (coin_entity, coin_transform, coin_hit_circle) in coins_query.iter() {
            let distance = player_transform
                .translation
                .add(player_hit_circle.offset)
                .distance(coin_transform.translation.add(coin_hit_circle.offset));
            if distance < (player_hit_circle.r + coin_hit_circle.r) {
                commands.entity(coin_entity).despawn_recursive();
            }
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO: animated sprite https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs
    // TODO: bundle it all as "player" bundle? is there a way to define what components entities should have?
    // TODO: is there a way to operate on Vec2 everywhere, but be OK with API expecting Vec3?
    let hit_circle = HitCircle {
        r: 4.,
        offset: vec3(-0.5, 0.5, 0.),
    };
    let transform = Transform::from_xyz(0., -TOPBAR_H / 2., Z_LAYER_SPRITES_PLAYER);
    let mut parent_command = commands.spawn((
        SpriteSheetBundle {
            // TODO: reorganize game area position calculations
            // TODO: add helpers for translating from window-centered coors to game area coords
            transform,
            // texture_atlas: sprite_sheet_texture_atlas_handle,
            texture_atlas: sprite_sheet.texture_atlas_handle.clone().unwrap(),
            sprite: TextureAtlasSprite {
                index: 19,
                anchor: Anchor::Center,
                ..default()
            },
            ..default()
        },
        SpritePadding {
            right: 1,
            bottom: 1,
            ..default()
        },
        hit_circle.clone(),
        ControlledDirection::Right,
        Player,
    ));

    #[cfg(debug_assertions)]
    parent_command.with_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(hit_circle.r).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(vec3(
                hit_circle.offset.x,
                hit_circle.offset.y,
                Z_LAYER_DEBUG_HIT_CIRCLES - Z_LAYER_SPRITES_PLAYER,
            )),
            ..default()
        });
    });
}

fn spawn_coin(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    // TODO: animated coin https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs
    // TODO: bundle it all as "coin" bundle? is there a way to define what components entities should have?
    let hit_circle = HitCircle {
        r: 3.5,
        offset: vec3(0., 0., 0.),
    };
    let transform = Transform::from_xyz(
        rng.gen_range(-40.0..40.0),
        -TOPBAR_H / 2. + rng.gen_range(-40.0..40.0),
        Z_LAYER_SPRITES_COINS,
    );
    let mut parent_command = commands.spawn((
        SpriteSheetBundle {
            // TODO: reorganize game area position calculations
            // TODO: Z>0 for layering?
            // TODO: add helpers for translating from window-centered coors to game area coords
            // TODO: TEMPORARY COORDS
            transform,
            texture_atlas: sprite_sheet.texture_atlas_handle.clone().unwrap(),
            sprite: TextureAtlasSprite {
                index: 0,
                anchor: Anchor::Center,
                ..default()
            },
            ..default()
        },
        SpritePadding {
            left: 1,
            right: 1,
            top: 1,
            bottom: 1,
        },
        hit_circle.clone(),
        Coin,
    ));

    #[cfg(debug_assertions)]
    parent_command.with_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(hit_circle.r).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(vec3(
                hit_circle.offset.x,
                hit_circle.offset.y,
                Z_LAYER_DEBUG_HIT_CIRCLES - Z_LAYER_SPRITES_COINS,
            )),
            ..default()
        });
    });
}

fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut ControlledDirection>,
) {
    // TODO: handle a case of pressed multiple arrows at once
    if keyboard_input.just_pressed(KeyCode::Left) {
        for mut controlled_direction in query.iter_mut() {
            *controlled_direction = ControlledDirection::Left;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        for mut controlled_direction in query.iter_mut() {
            *controlled_direction = ControlledDirection::Right;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        for mut controlled_direction in query.iter_mut() {
            *controlled_direction = ControlledDirection::Up;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        for mut controlled_direction in query.iter_mut() {
            *controlled_direction = ControlledDirection::Down;
        }
    }
}

fn fixed_update_player_sprite(mut query: Query<(&ControlledDirection, &mut TextureAtlasSprite)>) {
    for (controlled_direction, mut sprite) in query.iter_mut() {
        sprite.index = match *controlled_direction {
            ControlledDirection::Up => 18,
            ControlledDirection::Right => 19,
            ControlledDirection::Down => 20,
            ControlledDirection::Left => 21,
        };
    }
}

fn fixed_update_player(
    mut query: Query<(&ControlledDirection, &mut Transform, Option<&SpritePadding>)>,
) {
    // TODO: is it possible to bind speed to FPS (change in FPS -> automatic change of speed to make it constant in result), without allowing for non-integers?
    const MOVEMENT_PER_FRAME: f32 = 2.;

    for (controlled_direction, mut transform, maybe_sprite_padding) in query.iter_mut() {
        match controlled_direction {
            ControlledDirection::Left => transform.translation.x -= MOVEMENT_PER_FRAME,
            ControlledDirection::Right => transform.translation.x += MOVEMENT_PER_FRAME,
            ControlledDirection::Up => transform.translation.y += MOVEMENT_PER_FRAME,
            ControlledDirection::Down => transform.translation.y -= MOVEMENT_PER_FRAME,
        }

        let sprite_padding = maybe_sprite_padding
            .copied()
            .unwrap_or(SpritePadding::default());

        transform.translation.x = transform.translation.x.clamp(
            -GAME_AREA_W / 2. + SPRITE_SHEET_SPRITE_W / 2. - sprite_padding.left as f32,
            GAME_AREA_W / 2. - SPRITE_SHEET_SPRITE_W / 2. + sprite_padding.right as f32,
        );
        transform.translation.y = transform.translation.y.clamp(
            -GAME_AREA_H / 2. - TOPBAR_H / 2. + SPRITE_SHEET_SPRITE_H / 2.
                - sprite_padding.bottom as f32,
            GAME_AREA_H / 2. - TOPBAR_H / 2. - SPRITE_SHEET_SPRITE_H / 2.
                + sprite_padding.top as f32,
        );
    }
}

// TODO: tests 1 https://github.com/bevyengine/bevy/blob/latest/tests/how_to_test_systems.rs
// TODO: tests 2 https://bevy-cheatbook.github.io/programming/system-tests.html
