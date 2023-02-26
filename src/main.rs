use std::ops::Add;
use std::time::Duration;

use bevy::ecs::schedule::ShouldRun;
#[allow(unused_imports)]
use bevy::math::{vec2, vec3, Vec3Swizzles};
use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use iyes_loopless::prelude::AppLooplessFixedTimestepExt;
use rand::Rng;

use crate::pico8color::Pico8Color;

// TODO: should I "mod" my modules in lib.rs instead of main.rs?
mod pico8color;

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

// TODO: z-index https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs

// TODO: scaling https://github.com/bevyengine/bevy/blob/latest/examples/window/scale_factor_override.rs
// TODO: window resizing https://github.com/bevyengine/bevy/blob/latest/examples/window/window_resizing.rs
// TODO: window settings https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs

// TODO: logs https://github.com/bevyengine/bevy/blob/latest/examples/app/logs.rs

// TODO: modules, functions, scopes
// TODO: plugins 1 https://github.com/bevyengine/bevy/blob/latest/examples/app/plugin.rs
// TODO: plugins 2 https://github.com/bevyengine/bevy/blob/latest/examples/app/plugin_group.rs

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

// TODO: bevy::sprite::collide_aabb::collide(…)

// TODO: app.add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(0.5)).with_system(…))

// TODO consider system sets or system labels for making sure input is handled first, then update, then draw, then removal of dead entities
//      - res 1 : https://bevy-cheatbook.github.io/programming/system-order.html
//      - res 2 : https://bevy-cheatbook.github.io/programming/system-sets.html

// TODO game states https://bevy-cheatbook.github.io/programming/states.html

const FIXED_FPS: u64 = 30;

const FIXED_TIMESTEP_GAME_LOOP: &str = "fixed_timestep_game_loop";

const SPRITE_SHEET_SPRITE_W: f32 = 8.;
const SPRITE_SHEET_SPRITE_H: f32 = 8.;

const Z_LAYER_GAME_AREA: f32 = 1.;
const Z_LAYER_DEBUG_HIT_CIRCLES: f32 = Z_LAYER_GAME_AREA + 1.;
const Z_LAYER_SPRITES_COINS: f32 = Z_LAYER_DEBUG_HIT_CIRCLES + 1.;
const Z_LAYER_SPRITES_PLAYER: f32 = Z_LAYER_SPRITES_COINS + 1.;
#[cfg(debug_assertions)]
const Z_LAYER_DEBUG_SPRITE_BOUNDARIES: f32 = Z_LAYER_SPRITES_PLAYER + 1.;

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
    // TODO: group both plugins below into a single plugin for pixel upscaling camera
    .add_plugin(bevy_pixel_camera::PixelCameraPlugin)
    .add_plugin(bevy_pixel_camera::PixelBorderPlugin {
        color: bevy_color_from(Pico8Color::Black),
    });

    // TODO merge both FPS-related diagnostics lines into a single well-named plugin
    // Print FPS in a console
    #[cfg(debug_assertions)]
    app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());

    #[cfg(debug_assertions)]
    app.add_plugin(bevy_prototype_debug_lines::DebugLinesPlugin::default());

    // Get rid of edges of neighbour sprites visible around the given sprite from the sprite sheet
    app.insert_resource(Msaa { samples: 1 })
        // Draw a solid background color
        .insert_resource(ClearColor(bevy_color_from(Pico8Color::Black)));

    app.init_resource::<SpriteSheet>()
        .add_startup_system(load_spritesheet)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_game_area)
        // TODO: will it affect HTML embedded game?
        .add_system(bevy::window::close_on_esc)
        .add_system(handle_keyboard_input);

    #[cfg(debug_assertions)]
    app.add_system(draw_debug_sprite_boundaries);

    app.add_fixed_timestep(
        Duration::from_nanos(1_000_000_000 / FIXED_FPS),
        FIXED_TIMESTEP_GAME_LOOP,
    )
    .add_fixed_timestep_child_stage(FIXED_TIMESTEP_GAME_LOOP)
    .add_fixed_timestep_child_stage(FIXED_TIMESTEP_GAME_LOOP)
    .add_fixed_timestep_child_stage(FIXED_TIMESTEP_GAME_LOOP);

    #[cfg(debug_assertions)]
    app.add_fixed_timestep_system(FIXED_TIMESTEP_GAME_LOOP, 0, debug_fixed);

    app.add_fixed_timestep_system_set(
        FIXED_TIMESTEP_GAME_LOOP,
        1,
        SystemSet::new()
            .with_run_criteria(run_if_there_is_no_player)
            .with_system(spawn_player),
    )
    .add_fixed_timestep_system_set(
        FIXED_TIMESTEP_GAME_LOOP,
        1,
        SystemSet::new()
            .with_run_criteria(run_if_there_is_no_coin)
            .with_system(spawn_coin),
    );

    app.add_fixed_timestep_system(FIXED_TIMESTEP_GAME_LOOP, 2, fixed_update_player)
        .add_fixed_timestep_system(FIXED_TIMESTEP_GAME_LOOP, 2, fixed_update_player_sprite);

    app.add_fixed_timestep_system(FIXED_TIMESTEP_GAME_LOOP, 3, coin_pickup);

    app.run();
}

// Copied from https://github.com/IyesGames/iyes_loopless#fixed-timestep-control
#[cfg(debug_assertions)]
fn debug_fixed(timesteps: Res<iyes_loopless::fixedtimestep::FixedTimesteps>) {
    let info = timesteps.get_current().unwrap();
    debug!(
        "Fixed timestep: expected = {:?} | overstepped by = {:?}",
        info.timestep(),
        info.remaining(),
    );
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
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

// TODO: move to some helper module
fn bevy_color_from(pico8_color: Pico8Color) -> Color {
    Color::hex(pico8_color.hex()).unwrap()
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
            color: bevy_color_from(Pico8Color::DarkBlue),
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
    players_query: Query<(&Player, &Transform, &HitCircle)>,
    coins_query: Query<(Entity, &Coin, &Transform, &HitCircle)>,
) {
    for (_, player_transform, player_hit_circle) in players_query.iter() {
        for (coin_entity, _, coin_transform, coin_hit_circle) in coins_query.iter() {
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
    #[allow(unused_mut)] mut meshes: ResMut<Assets<Mesh>>,
    #[allow(unused_mut)] mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO: animated sprite https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs
    // TODO: bundle it all as "player" bundle? is there a way to define what components entities should have?
    // TODO: is there a way to operate on Vec2 everywhere, but be OK with API expecting Vec3?
    let hit_circle = HitCircle {
        r: 4.,
        offset: vec3(-0.5, 0.5, 0.),
    };
    let transform = Transform::from_xyz(0., -TOPBAR_H / 2., Z_LAYER_SPRITES_PLAYER);
    #[allow(unused_mut)]
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

    #[allow(unused_mut, unused_variables)]
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
    #[allow(unused_mut, unused_variables)] mut meshes: ResMut<Assets<Mesh>>,
    #[allow(unused_mut, unused_variables)] mut materials: ResMut<Assets<ColorMaterial>>,
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
    #[allow(unused_mut, unused_variables)]
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

#[cfg(debug_assertions)]
fn draw_debug_sprite_boundaries(
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    query: Query<(&Transform, &TextureAtlasSprite)>,
) {
    for (transform, _sprite) in query.iter() {
        // TODO: can we do it in a shorter way?
        lines.line(
            transform.translation.xyz().add(vec3(
                SPRITE_SHEET_SPRITE_W / 2.,
                SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            transform.translation.xyz().add(vec3(
                -SPRITE_SHEET_SPRITE_W / 2.,
                SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            0.,
        );
        lines.line(
            transform.translation.xyz().add(vec3(
                -SPRITE_SHEET_SPRITE_W / 2.,
                SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            transform.translation.xyz().add(vec3(
                -SPRITE_SHEET_SPRITE_W / 2.,
                -SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            0.,
        );
        lines.line(
            transform.translation.xyz().add(vec3(
                -SPRITE_SHEET_SPRITE_W / 2.,
                -SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            transform.translation.xyz().add(vec3(
                SPRITE_SHEET_SPRITE_W / 2.,
                -SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            0.,
        );
        lines.line(
            transform.translation.xyz().add(vec3(
                SPRITE_SHEET_SPRITE_W / 2.,
                -SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            transform.translation.xyz().add(vec3(
                SPRITE_SHEET_SPRITE_W / 2.,
                SPRITE_SHEET_SPRITE_H / 2.,
                Z_LAYER_DEBUG_SPRITE_BOUNDARIES,
            )),
            0.,
        );
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
