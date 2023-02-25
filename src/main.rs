use std::time::Duration;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use iyes_loopless::prelude::AppLooplessFixedTimestepExt;

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

// TODO: make it not a constant, but some entity's property
const PLAYER_W: f32 = 8.;
const PLAYER_H: f32 = 8.;

const VIEWPORT_W: f32 = GAME_AREA_W;
const VIEWPORT_H: f32 = TOPBAR_H + GAME_AREA_H;

// TODO: bevy::sprite::collide_aabb::collide(…)

// TODO: app.add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(0.5)).with_system(…))

// TODO consider system sets or system labels for making sure input is handled first, then update, then draw, then removal of dead entities
//      - res 1 : https://bevy-cheatbook.github.io/programming/system-order.html
//      - res 2 : https://bevy-cheatbook.github.io/programming/system-sets.html

// TODO game states https://bevy-cheatbook.github.io/programming/states.html

const FIXED_FPS: u64 = 60;

const FIXED_TIMESTEP_GAME_LOOP: &str = "fixed_timestep_game_loop";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: GAME_TITLE.to_string(),
                        // width: SCALE * VIEWPORT_W,
                        // height: SCALE * VIEWPORT_H,
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
        })
        // TODO merge both FPS-related diagnostics lines into a single well-named plugin
        // Print FPS in a console
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // Get rid of edges of neighbour sprites visible around the given sprite from the sprite sheet
        .insert_resource(Msaa { samples: 1 })
        // Draw a solid background color
        .insert_resource(ClearColor(bevy_color_from(Pico8Color::Black)))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_game_area)
        .add_startup_system(spawn_player)
        // TODO: will it affect HTML embedded game?
        .add_system(bevy::window::close_on_esc)
        .add_system(handle_keyboard_input)
        .add_fixed_timestep(
            Duration::from_nanos(1_000_000_000 / FIXED_FPS),
            FIXED_TIMESTEP_GAME_LOOP,
        )
        .add_fixed_timestep_system(FIXED_TIMESTEP_GAME_LOOP, 0, debug_fixed)
        .add_fixed_timestep_system(FIXED_TIMESTEP_GAME_LOOP, 0, fixed_update_player)
        .add_fixed_timestep_system(FIXED_TIMESTEP_GAME_LOOP, 0, fixed_update_player_sprite)
        .run();
}

// Copied from https://github.com/IyesGames/iyes_loopless#fixed-timestep-control
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

#[derive(Component, Default)]
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
        transform: Transform::from_xyz(-GAME_AREA_W / 2., GAME_AREA_H / 2. - TOPBAR_H / 2., 0.),
        ..default()
    });
}

// TODO: note it down somewhere or make sure it is satisfied in the code itself
//       (taken from bevy_pixel_camera plugin's README):
//       Note that if either the width or the height of your sprite is not divisible by 2,
//       you need to change the anchor of the sprite (which is at the center by default),
//       or it will not be pixel aligned.

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // TODO: move atlas creation out of the system. Atlas will be used by many entities, possibly created in their separate setup systems
    let sprite_sheet_handle: Handle<Image> = asset_server.load("spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(sprite_sheet_handle, vec2(8., 8.), 16, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // TODO: animated sprite https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs
    // TODO: change sprite according to direction
    // TODO: bundle it all as "player" bundle? is there a way to define what components entities should have?
    commands.spawn((
        SpriteSheetBundle {
            // TODO: reorganize game area position calculations
            // TODO: Z>0 for layering?
            // TODO: add helpers for translating from window-centered coors to game area coords
            transform: Transform::from_xyz(0., -TOPBAR_H / 2., 0.),
            texture_atlas: texture_atlas_handle,
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
        ControlledDirection::Right,
    ));
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
    const MOVEMENT_PER_FRAME: f32 = 1.;

    for (controlled_direction, mut transform, maybe_sprite_padding) in query.iter_mut() {
        match controlled_direction {
            ControlledDirection::Left => transform.translation.x -= MOVEMENT_PER_FRAME,
            ControlledDirection::Right => transform.translation.x += MOVEMENT_PER_FRAME,
            ControlledDirection::Up => transform.translation.y += MOVEMENT_PER_FRAME,
            ControlledDirection::Down => transform.translation.y -= MOVEMENT_PER_FRAME,
        }

        let default_sprite_padding = SpritePadding::default();
        let sprite_padding = maybe_sprite_padding.unwrap_or(&default_sprite_padding);
        transform.translation.x = transform.translation.x.clamp(
            -GAME_AREA_W / 2. + PLAYER_W / 2. - sprite_padding.left as f32,
            GAME_AREA_W / 2. - PLAYER_W / 2. + sprite_padding.right as f32,
        );
        transform.translation.y = transform.translation.y.clamp(
            -GAME_AREA_H / 2. - TOPBAR_H / 2. + PLAYER_H / 2. - sprite_padding.bottom as f32,
            GAME_AREA_H / 2. - TOPBAR_H / 2. - PLAYER_H / 2. + sprite_padding.top as f32,
        );
    }
}

// TODO: tests 1 https://github.com/bevyengine/bevy/blob/latest/tests/how_to_test_systems.rs
// TODO: tests 2 https://bevy-cheatbook.github.io/programming/system-tests.html
