use bevy::math::vec2;
use bevy::prelude::*;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(
            Color::hex("1d2b53").unwrap_or(Color::MIDNIGHT_BLUE),
        ))
        // Get rid of edges of neighbour sprites visible around the given sprite from the sprite sheet
        // (based on https://discord.com/channels/691052431525675048/1038900941795102770/1038940595714002964)
        .insert_resource(Msaa { samples: 1 })
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
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
    commands.spawn(Camera2dBundle::default());
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

    // TODO: change sprite according to direction
    commands.spawn((
        SpriteSheetBundle {
            // TODO: center sprite on position
            // TODO: what initial XY to set?
            // TODO: Z>0 for layering?
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(8.)),
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(19),
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
        // TODO: defined game arena size (and scale it within viewport?)
        transform.translation.y = transform.translation.y.clamp(-200., 200.);
        transform.translation.x = transform.translation.x.clamp(-300., 300.);
    }
}
