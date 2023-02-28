use std::ops::Add;

use bevy::prelude::*;

use crate::game::collision::HitCircle;
use crate::game::player::Player;
use crate::game::Coin;

pub fn create_collect_coins_systems() -> SystemSet {
    SystemSet::new().with_system(collect_coins)
}

fn collect_coins(
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
