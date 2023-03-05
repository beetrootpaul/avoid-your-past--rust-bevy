use std::ops::Add;

use bevy::prelude::*;

use crate::game::audio::AudioFiles;
use crate::game::collision::HitCircle;
use crate::game::game_state::GameState;
use crate::game::player::Player;
use crate::game::Coin;

pub fn create_systems_collect_coins() -> SystemSet {
    SystemSet::new()
    // ConditionSet::new()
    //     .run_if(GameState::should_game_update)
    //     .with_system(collect_coins)
    //     .into()
}

fn collect_coins(
    mut commands: Commands,
    players_query: Query<(&Transform, &HitCircle), With<Player>>,
    coins_query: Query<(Entity, &Transform, &HitCircle), With<Coin>>,
    // audio: Res<Audio>,
    audio_files: Res<AudioFiles>,
) {
    for (player_transform, player_hit_circle) in players_query.iter() {
        for (coin_entity, coin_transform, coin_hit_circle) in coins_query.iter() {
            let distance = player_transform
                .translation
                .add(player_hit_circle.offset)
                .distance(coin_transform.translation.add(coin_hit_circle.offset));
            if distance < (player_hit_circle.r + coin_hit_circle.r) {
                commands.entity(coin_entity).despawn_recursive();
                // TODO: consider moving audio play somewhere else, and here just a simple function call or event maybe?
                // let sfx = audio_files
                //     .sfx_coin_collected
                //     .clone()
                //     .expect("should have sfx_coin_collected file already loaded");
                // audio.play(sfx);
            }
        }
    }
}
