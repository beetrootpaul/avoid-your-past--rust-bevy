use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::camera::CameraProjection;
use bevy::sprite::Anchor;

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

// TODO: adapt SCALE according to window size (for whole integer multipliers)
const SCALE: f32 = 4.;

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

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: GAME_TITLE.to_string(),
                        width: SCALE * VIEWPORT_W,
                        height: SCALE * VIEWPORT_H,
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
        // TODO merge both FPS-related diagnostics lines into a single well-named plugin
        // Print FPS in a console
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // Get rid of edges of neighbour sprites visible around the given sprite from the sprite sheet
        .insert_resource(Msaa { samples: 1 })
        // TODO: make all colors defined as a PICO-8 palette
        // Draw a solid background color
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_game_area)
        .add_startup_system(spawn_player)
        // TODO: will it affect HTML embedded game?
        .add_system(bevy::window::close_on_esc)
        .add_system(handle_keyboard_input)
        .add_system(update_controlled_directions)
        .run();
}

#[derive(Component)]
enum ControlledDirection {
    Left,
    Right,
    Up,
    Down,
}

fn spawn_camera(mut commands: Commands) {
    // TODO: how to do it better? It was a simple `Camera2dBundle::default()` before I wanted to define a `scale`
    // TODO: change coords to start top-left. Useful example: https://bevy-cheatbook.github.io/cookbook/custom-projection.html
    // TODO: simplify this camera setup. See https://bevy-cheatbook.github.io/cookbook/custom-projection.html
    let far = 1000.0_f32;
    let projection = OrthographicProjection {
        far,
        scale: 1. / SCALE,
        ..Default::default()
    };
    let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);
    let view_projection = projection.get_projection_matrix() * transform.compute_matrix().inverse();
    let frustum = bevy::render::primitives::Frustum::from_view_projection(
        &view_projection,
        &transform.translation,
        &transform.back(),
        projection.far(),
    );
    let camera2d_bundle = Camera2dBundle {
        camera_render_graph: bevy::render::camera::CameraRenderGraph::new("core_2d"),
        projection,
        visible_entities: bevy::render::view::VisibleEntities::default(),
        frustum,
        transform,
        global_transform: Default::default(),
        camera: Camera::default(),
        camera_2d: Camera2d::default(),
        tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::Disabled,
    };
    commands.spawn(camera2d_bundle);
}

fn spawn_game_area(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            // TODO: make all colors defined as a PICO-8 palette
            color: Color::hex("1d2b53").unwrap_or(Color::MIDNIGHT_BLUE),
            custom_size: Some(vec2(GAME_AREA_W, GAME_AREA_H)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        transform: Transform::from_xyz(-GAME_AREA_W / 2., GAME_AREA_H / 2. - TOPBAR_H / 2., 0.),
        ..default()
    });
}

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

fn update_controlled_directions(
    time: Res<Time>,
    mut query: Query<(&ControlledDirection, &mut Transform)>,
) {
    // TODO: desired speed: 2 px every 1/30s (PICO-8 game was in 30fps)
    const SPEED: f32 = 200.;
    for (controlled_direction, mut transform) in query.iter_mut() {
        match controlled_direction {
            ControlledDirection::Left => transform.translation.x -= SPEED * time.delta_seconds(),
            ControlledDirection::Right => transform.translation.x += SPEED * time.delta_seconds(),
            ControlledDirection::Up => transform.translation.y += SPEED * time.delta_seconds(),
            ControlledDirection::Down => transform.translation.y -= SPEED * time.delta_seconds(),
        }
        transform.translation.x = transform.translation.x.clamp(
            -GAME_AREA_W / 2. + PLAYER_W / 2.,
            GAME_AREA_W / 2. - PLAYER_W / 2.,
        );
        transform.translation.y = transform.translation.y.clamp(
            -GAME_AREA_H / 2. - TOPBAR_H / 2. + PLAYER_H / 2.,
            GAME_AREA_H / 2. - TOPBAR_H / 2. - PLAYER_H / 2.,
        );
    }
}

// TODO: tests 1 https://github.com/bevyengine/bevy/blob/latest/tests/how_to_test_systems.rs
// TODO: tests 2 https://bevy-cheatbook.github.io/programming/system-tests.html
