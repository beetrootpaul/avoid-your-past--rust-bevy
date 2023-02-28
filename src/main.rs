use bevy::prelude::*;
use bevy::window::close_on_esc;

use crate::game::GamePlugin;
#[cfg(debug_assertions)]
use crate::print_fps::PrintFpsPlugin;

mod game;
mod pico8_color;
mod pixel_art_support;
mod print_fps;
mod z_layer;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        bevy::DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: game::GAME_TITLE.to_string(),
                    ..default()
                },
                ..default()
            })
            .set(AssetPlugin {
                // Watch for changes made to assets while the app is run and
                // hot-reload them in the app without need to run the app again
                watch_for_changes: true,
                ..default()
            })
            // Prevent blurring of scaled up pixel art sprites
            .set(ImagePlugin::default_nearest()),
    );

    // Get rid of edges of neighbour sprites visible around the given sprite from the sprite sheet
    app.insert_resource(Msaa { samples: 1 });

    app.add_plugin(GamePlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(PrintFpsPlugin);

    #[cfg(debug_assertions)]
    app.add_system(close_on_esc);

    app.run();
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

// TODO: audio resource 1: https://github.com/bevyengine/bevy/blob/latest/examples/audio/audio.rs
// TODO: audio resource 2: https://github.com/bevyengine/bevy/blob/latest/examples/audio/audio_control.rs

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
