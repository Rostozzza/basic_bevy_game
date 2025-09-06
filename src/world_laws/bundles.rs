use bevy::prelude::*;
use crate::world_laws::components::*;

#[derive(Bundle)]
pub struct KinematicBodyBundle {
    pub gravity: Gravity,
    pub mass: Mass,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub forces: Forces,
}

impl Default for KinematicBodyBundle {
    fn default() -> Self {
        Self {
            gravity: Gravity,
            mass: Mass(1.0),
            velocity: Velocity(Vec3::ZERO),
            acceleration: Acceleration(Vec3::ZERO),
            forces: Forces(vec![]),
        }
    }
}