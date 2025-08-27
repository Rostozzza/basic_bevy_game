use bevy::prelude::*;
use bevy::core_pipeline::core_2d::Camera2d;

use crate::world_laws::systems::*;
use crate::world_laws::components::*;

pub mod world_laws;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            apply_gravity,
            change_acceleration_by_forces_system,
            acceleration_system,
            move_by_velocity_system,
            ttl_system,
        ))
        .run();
}

fn setup(mut commands: Commands)
{
    commands.spawn(Camera2d::default());
    
    let sprite = Sprite {
        color: Color::srgb(0.3, 0.7, 0.9),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    };
    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));

    commands.spawn((sprite, transform, Gravity, Mass(1.0), Velocity(Vec3::ZERO), Acceleration(Vec3::ZERO), Forces(vec![])));
}