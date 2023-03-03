use bevy::ecs::schedule::ShouldRun;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::game::collision::HitCircle;
#[cfg(debug_assertions)]
use crate::game::collision_debug::create_hit_circle_visualization;
use crate::game::game_area::{GAME_AREA_H, GAME_AREA_W};
use crate::game::gui::TOPBAR_H;
use crate::game::sprites::{SpriteDimensions, SpriteSheet};
use crate::game::trail::TrailOrigin;
use crate::pixel_art_support::FixedFpsTime;
use crate::z_layer::Z_LAYER_SPRITES_PLAYER;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub enum PlayerMovement {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite_sheet_bundle: SpriteSheetBundle,
    player_movement: PlayerMovement,
    sprite_dimensions: SpriteDimensions,
    hit_circle: HitCircle,
    trail_origin: TrailOrigin,
}

pub fn create_systems_player_spawn() -> SystemSet {
    SystemSet::new()
        .with_run_criteria(there_is_no_player)
        .with_system(spawn_player)
}

pub fn create_systems_player_move() -> SystemSet {
    SystemSet::new()
        .with_system(move_player)
        .with_system(update_player_sprite)
}

fn there_is_no_player(query: Query<&Player>) -> ShouldRun {
    if query.iter().count() > 0 {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

fn spawn_player(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    fixed_fps_time: Res<FixedFpsTime>,
) {
    let initial_movement = PlayerMovement::Right;

    let hit_circle = HitCircle {
        // TODO: make r 3.5, then offset -0.3 in x for right facing movement and similar adjustments for other directions.
        //       Consider incorporating sprite dimensions to calculate sprite center (as an impl on a trait?)
        r: 3.9,
        offset: vec3(-0.5, 0.5, 0.),
    };

    let mut parent_command = commands.spawn(PlayerBundle {
        player: Player,
        sprite_sheet_bundle: SpriteSheetBundle {
            // TODO: reorganize game area position calculations
            // TODO: add helpers for translating from window-centered coors to game area coords
            transform: Transform::from_xyz(0., -TOPBAR_H / 2., Z_LAYER_SPRITES_PLAYER),
            texture_atlas: sprite_sheet.texture_atlas_handle.clone().unwrap(),
            sprite: TextureAtlasSprite {
                index: get_sprite_index_for_movement(&initial_movement),
                anchor: Anchor::Center,
                ..default()
            },
            ..default()
        },
        player_movement: initial_movement,
        sprite_dimensions: SpriteDimensions {
            padding_right: 1.,
            padding_bottom: 1.,
            ..default()
        },
        hit_circle: hit_circle.clone(),
        // TODO: express time in frames, instead of seconds maybe?
        trail_origin: TrailOrigin::with_seconds_between_particles(
            4. * fixed_fps_time.duration.as_secs_f32(),
        ),
    });

    #[cfg(debug_assertions)]
    parent_command.with_children(|parent| {
        parent.spawn(create_hit_circle_visualization(
            &hit_circle,
            Z_LAYER_SPRITES_PLAYER,
            meshes,
            materials,
        ));
    });
}

fn move_player(mut query: Query<(&PlayerMovement, &mut Transform, Option<&SpriteDimensions>)>) {
    // TODO: is it possible to bind speed to FPS (change in FPS -> automatic change of speed to make it constant in result), without allowing for non-integers?
    const MOVEMENT_PER_FRAME: f32 = 2.;

    for (player_movement, mut transform, maybe_sprite_dimensions) in query.iter_mut() {
        match player_movement {
            PlayerMovement::Left => transform.translation.x -= MOVEMENT_PER_FRAME,
            PlayerMovement::Right => transform.translation.x += MOVEMENT_PER_FRAME,
            PlayerMovement::Up => transform.translation.y += MOVEMENT_PER_FRAME,
            PlayerMovement::Down => transform.translation.y -= MOVEMENT_PER_FRAME,
        }

        let sprite_dimensions = maybe_sprite_dimensions
            .copied()
            .unwrap_or(SpriteDimensions::default());

        transform.translation.x = transform.translation.x.clamp(
            -GAME_AREA_W / 2. + sprite_dimensions.width / 2. - sprite_dimensions.padding_left,
            GAME_AREA_W / 2. - sprite_dimensions.width / 2. + sprite_dimensions.padding_right,
        );
        transform.translation.y = transform.translation.y.clamp(
            -GAME_AREA_H / 2. - TOPBAR_H / 2. + sprite_dimensions.height / 2.
                - sprite_dimensions.padding_bottom,
            GAME_AREA_H / 2. - TOPBAR_H / 2. - sprite_dimensions.height / 2.
                + sprite_dimensions.padding_top,
        );
    }
}

fn update_player_sprite(mut query: Query<(&PlayerMovement, &mut TextureAtlasSprite)>) {
    for (player_movement, mut sprite) in query.iter_mut() {
        sprite.index = get_sprite_index_for_movement(player_movement);
    }
}

fn get_sprite_index_for_movement(movement: &PlayerMovement) -> usize {
    match *movement {
        PlayerMovement::Up => SpriteSheet::PLAYER_UP,
        PlayerMovement::Right => SpriteSheet::PLAYER_RIGHT,
        PlayerMovement::Down => SpriteSheet::PLAYER_DOWN,
        PlayerMovement::Left => SpriteSheet::PLAYER_LEFT,
    }
}
