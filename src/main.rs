use std::fs;
use json::*;

use bevy::prelude::*;
use bevy::core_pipeline::core_2d::Camera2d;

use crate::world_laws::systems::*;
use crate::world_laws::components::*;
use crate::world_laws::bundles::*;

use crate::bundles::*;

mod world_laws;

mod components;
mod systems;
mod bundles;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            setup_world, 
            setup
        ))
        .add_systems(Update, (
            apply_gravity,
            change_acceleration_by_forces_system,
            acceleration_system,
            move_by_velocity_system,
            ttl_system,
            collision_system,
        ))
        .run();
}

fn setup_world(mut commands: Commands)
{
    let json_text = fs::read_to_string("assets/world.json").unwrap();
    let world_data = json::parse(&json_text).unwrap();

    for ground in world_data["world"]["ground"].members() {
        let position = Vec2::new(
            ground["position"]["x"].as_f64().unwrap() as f32,
            ground["position"]["y"].as_f64().unwrap() as f32,
        );
        let size = Vec2::new(
            ground["size"]["width"].as_f64().unwrap() as f32,
            ground["size"]["height"].as_f64().unwrap() as f32,
        );
        let color = Color::from(Srgba::hex(ground["color"].as_str().unwrap()).unwrap());

        commands.spawn((
            ColliderShape
            {
                points: vec![
                    Vec2::new(-size.x / 2.0, -size.y / 2.0),
                    Vec2::new(size.x / 2.0, -size.y / 2.0),
                    Vec2::new(size.x / 2.0, size.y / 2.0),
                    Vec2::new(-size.x / 2.0, size.y / 2.0),
                ],
            },
            Sprite
            {
                color: color,
                custom_size: Some(size),
                ..default()
            },
            Transform::from_translation(position.extend(0.0)),
        ));
        // println!("Spawned ground entity at position: {:?}", position);
        // println!("Ground entity size: {:?}", size);
        // println!("Ground entity color: {:?}", color);
    }
}

fn setup(mut commands: Commands)
{
    commands.spawn(Camera2d::default());

    commands.spawn((
        EntityBundle
        {
            kinematics: KinematicBodyBundle
            {
                velocity: Velocity(Vec3::new(50.0, 50.0, 0.0)),
                ..default()
            },
            ..default()
        },
        ColliderShape
        {
            points: vec![
                Vec2::new(-10.0, -10.0),
                Vec2::new(10.0, -10.0),
                Vec2::new(10.0, 10.0),
                Vec2::new(-10.0, 10.0),
            ],
        }
    ));
}