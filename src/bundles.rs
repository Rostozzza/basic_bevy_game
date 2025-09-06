use bevy::prelude::*;

use crate::world_laws::bundles::*;

#[derive(Bundle)]
pub struct EntityBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub kinematics: KinematicBodyBundle,
}

impl Default for EntityBundle {
    fn default() -> Self {
        Self
        {
            sprite: Sprite {
                color: Color::srgb(0.3, 0.7, 0.9),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            kinematics: KinematicBodyBundle::default()
        }
    }
}
