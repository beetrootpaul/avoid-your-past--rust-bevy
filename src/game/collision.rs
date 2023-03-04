use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct HitCircle {
    pub r: f32,
    pub offset: Vec3,
}
