use bevy::app::AppExit;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowResizeConstraints};
use bevy_pixels::{PixelsPlugin, PixelsResource, PixelsStage};
use image::EncodableLayout;

use crate::game::{GamePlugin, SpriteSheet, VIEWPORT_H, VIEWPORT_W};
use crate::print_fps::PrintFpsPlugin;

mod game;
mod pico8_color;
mod pixel_art_support;
mod print_fps;
mod z_layer;

#[derive(Component, Debug)]
struct Color(u8, u8, u8, u8);

fn main() {
    #[cfg(target_arch = "wasm32")]
    const ZOOM: f32 = 3.;
    #[cfg(not(target_arch = "wasm32"))]
    const ZOOM: f32 = 4.;

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: game::GAME_TITLE.to_string(),
            width: VIEWPORT_W as f32 * ZOOM,
            height: VIEWPORT_H as f32 * ZOOM,
            resize_constraints: WindowResizeConstraints {
                min_width: VIEWPORT_W as f32,
                min_height: VIEWPORT_H as f32,
                ..default()
            },
            fit_canvas_to_parent: true,
            ..default()
        },
        ..default()
    }));

    app.add_plugin(PixelsPlugin {
        width: VIEWPORT_W as u32,
        height: VIEWPORT_H as u32,
        ..default()
    });

    app.add_startup_system(pixels_example_setup);
    // .add_system(pixels_example_bounce)
    // .add_system(pixels_example_movement.after(pixels_example_bounce))
    // .add_system_to_stage(PixelsStage::Draw, pixels_example_draw_background)
    // .add_system_to_stage(
    //     PixelsStage::Draw,
    //     pixels_example_draw_objects.after(pixels_example_draw_background),
    // );

    // app.add_plugins(
    //     DefaultPlugins
    //         .set(AssetPlugin {
    //             // Watch for changes made to assets while the app is run and
    //             // hot-reload them in the app without need to run the app again
    //             #[cfg(debug_assertions)]
    //             watch_for_changes: true,
    //             ..default()
    //         })
    //         // Prevent blurring of scaled up pixel art sprites
    //         // .set(ImagePlugin::default_nearest()),
    // );

    // Get rid of edges of neighbour sprites visible around the given sprite from the sprite sheet
    // app.insert_resource(Msaa { samples: 1 });

    app.add_plugin(GamePlugin);

    app.add_plugin(PrintFpsPlugin);

    #[cfg(debug_assertions)]
    app.add_system(close_on_esc);

    app.run();
}

#[derive(Bundle, Debug)]
struct PixelsExampleRectangleBundle {
    position: PixelsExamplePosition,
    velocity: PixelsExampleVelocity,
    size: PixelsExampleSize,
    color: Color,
}

#[derive(Component, Debug)]
struct PixelsExamplePosition {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct PixelsExampleVelocity {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct PixelsExampleSize {
    width: f32,
    height: f32,
}

fn pixels_example_setup(mut commands: Commands) {
    let box_object = PixelsExampleRectangleBundle {
        position: PixelsExamplePosition { x: 5., y: 5. },
        velocity: PixelsExampleVelocity { x: 0.3, y: 0.8 },
        size: PixelsExampleSize {
            width: 32.,
            height: 48.,
        },
        color: Color(0x5e, 0x48, 0xe8, 0xff),
    };
    commands.spawn(box_object);
}

fn pixels_example_bounce(
    mut query: Query<(
        &PixelsExamplePosition,
        &mut PixelsExampleVelocity,
        &PixelsExampleSize,
        &mut Color,
    )>,
) {
    for (position, mut velocity, size, mut color) in query.iter_mut() {
        let mut bounce = false;
        if position.x < 4. || position.x + size.width > (VIEWPORT_W - 4.) {
            velocity.x *= -1.;
            bounce = true;
        }
        if position.y < 4. || position.y + size.height > (VIEWPORT_H - 4.) {
            velocity.y *= -1.;
            bounce = true;
        }
    }
}

fn pixels_example_movement(mut query: Query<(&mut PixelsExamplePosition, &PixelsExampleVelocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        let mut x_tmp = position.x + velocity.x;
        x_tmp = x_tmp.max(1.);
        position.x = x_tmp;
        let mut y_tmp = (position.y + velocity.y);
        y_tmp = y_tmp.max(1.);
        position.y = y_tmp;
    }
}

fn pixels_example_draw_background(mut pixels_resource: ResMut<PixelsResource>) {
    let frame = pixels_resource.pixels.get_frame_mut();
    frame.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff].repeat(frame.len() / 4));
}

fn pixels_example_draw_objects(
    mut pixels_resource: ResMut<PixelsResource>,
    query: Query<(&PixelsExamplePosition, &PixelsExampleSize, &Color)>,
    sprite_sheet: ResMut<SpriteSheet>,
) {
    let frame = pixels_resource.pixels.get_frame_mut();
    let frame_width_bytes = (VIEWPORT_W as u32 * 4) as usize;

    for (position, size, color) in query.iter() {
        let x_offset = position.x as usize * 4;
        let width_bytes = size.width as usize * 4;
        let object_row = &[color.0, color.1, color.2, color.3].repeat(size.width as usize);

        for y in (position.y as usize)..(position.y as usize + size.height as usize - 1) {
            let y_offset = y * frame_width_bytes;
            let i = y_offset + x_offset;
            let j = i + width_bytes;

            frame[i..j].copy_from_slice(object_row);
        }
    }

    let frame_w: usize = VIEWPORT_W as usize;
    let frame_h: usize = VIEWPORT_H as usize;

    if let Some(rgba_image) = sprite_sheet.maybe_rgba_image.as_ref() {
        let sprite_w: usize = rgba_image.width() as usize;
        let sprite_h: usize = rgba_image.height() as usize;
        let sprite_bytes: &[u8] = rgba_image.as_bytes();

        // if let Some(rgba_bytes) = sprite_sheet.maybe_bytes.as_ref() {
        //     let sprite_w: usize = 128;
        //     let sprite_h: usize = 32;
        //     let sprite_bytes: &[u8] = rgba_bytes;

        // println!("{:?}", sprite_sheet.maybe_rgba_image.as_ref().unwrap().width());
        // println!("{:?}", sprite_sheet.maybe_rgba_image.as_ref().unwrap().height());
        // println!("{:?}", sprite_sheet.maybe_rgba_image.as_ref().unwrap().as_bytes());

        // for _ in 0..1_000 {
        for sprite_row in 0..sprite_h {
            // let target_range =
            //     (sprite_row * frame_w * 4)..(sprite_row * frame_w * 4 + sprite_w * 4);
            // let source_range =
            //     (sprite_row * sprite_w * 4)..(sprite_row * sprite_w * 4 + sprite_w * 4);
            // frame[target_range].copy_from_slice(&sprite_bytes[source_range]);

            for sprite_column in 0..sprite_w {
                let target_i_r = sprite_row * frame_w * 4 + sprite_column * 4;
                let target_i_g = target_i_r + 1;
                let target_i_b = target_i_g + 1;
                let target_i_a = target_i_b + 1;
                let source_i_r = sprite_row * sprite_w * 4 + sprite_column * 4;
                let source_i_g = source_i_r + 1;
                let source_i_b = source_i_g + 1;
                let source_i_a = source_i_b + 1;
                if sprite_bytes[source_i_a] > 0x88 {
                    // frame[target_i_r..=target_i_a].copy_from_slice(&sprite_bytes[source_i_r..=source_i_a]);

                    frame[target_i_r] = sprite_bytes[source_i_r];
                    frame[target_i_g] = sprite_bytes[source_i_g];
                    frame[target_i_b] = sprite_bytes[source_i_b];
                    frame[target_i_a] = sprite_bytes[source_i_a];
                }
            }
        }
        // }
    }

    let line_data = &[0xff, 0x00, 0x00, 0xff].repeat(frame_w - 2);
    let width_bytes = (frame_w - 2) * 4;
    let x_offset = 4;
    let y_offset = frame_width_bytes;
    let i = y_offset + x_offset;
    let j = i + width_bytes;
    frame[i..j].copy_from_slice(line_data);
    let y_offset = (frame_w - 2) * frame_width_bytes;
    let i = y_offset + x_offset;
    let j = i + width_bytes;
    frame[i..j].copy_from_slice(line_data);
}

// TODO: at the end, remove some "pub"s and exports and check which of them are still needed (or clippy will take care of that?)

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

// TODO: audio control: https://github.com/bevyengine/bevy/blob/latest/examples/audio/audio_control.rs

// TODO: fixed FPS 1 https://github.com/bevyengine/bevy/blob/latest/examples/ecs/fixed_timestep.rs
// TODO: fixed FPS 2 https://bevy-cheatbook.github.io/features/fixed-timestep.html
// TODO: app.add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(0.5)).with_system(…))

// TODO: logs https://github.com/bevyengine/bevy/blob/latest/examples/app/logs.rs

// TODO: game states 1 https://github.com/bevyengine/bevy/blob/latest/examples/ecs/state.rs
// TODO: game states 2 https://github.com/IyesGames/iyes_loopless
// TODO game states https://bevy-cheatbook.github.io/programming/states.html

// TODO: components on sparse sets: https://bevy-cheatbook.github.io/patterns/component-storage.html

// TODO add printing out all resource types (maybe use a proper logging method instead of printing one) https://bevy-cheatbook.github.io/cookbook/print-resources.html

// TODO consider system sets or system labels for making sure input is handled first, then update, then draw, then removal of dead entities
//      - res 1 : https://bevy-cheatbook.github.io/programming/system-order.html
//      - res 2 : https://bevy-cheatbook.github.io/programming/system-sets.html

// TODO: tests 1 https://github.com/bevyengine/bevy/blob/latest/tests/how_to_test_systems.rs
// TODO: tests 2 https://bevy-cheatbook.github.io/programming/system-tests.html

// TODO: pixel art rendering discord://discord.com/channels/691052431525675048/1075359868631928933/1075359868631928933
