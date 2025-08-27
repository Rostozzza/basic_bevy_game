use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Acceleration(pub Vec3);

#[derive(Component)]
pub struct TTL(pub f32);

pub struct Force(pub Vec3);

#[derive(Component)]
pub struct Forces(pub Vec<Force>);

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Gravity;