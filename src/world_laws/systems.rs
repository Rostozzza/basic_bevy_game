use bevy::prelude::*;
use crate::world_laws::components::*;

pub fn move_by_velocity_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>)
{
    for (mut transform, velocity) in &mut query
    {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

pub fn acceleration_system(time: Res<Time>, query: Query<(&Acceleration, &mut Velocity)>)
{
    for (acceleration, mut velocity) in query
    {
        velocity.0 += acceleration.0 * time.delta_secs();
    }
}

pub fn change_acceleration_by_forces_system(mut query: Query<(&mut Acceleration, &mut Forces, &Mass)>)
{
    for (mut acceleration, mut forces, mass) in &mut query
    {
        let mut sum_forces = Vec3::ZERO;
        for force in &forces.0
        {
            sum_forces += force.0;
        }
        if mass.0 <= 0.0
        {
            println!("Warning: Entity with zero or negative mass encountered. Skipping acceleration update.");
            continue;
        }
        acceleration.0 = sum_forces / mass.0;
        forces.0.clear();
    }
}

pub fn ttl_system(time: Res<Time>, mut commands: Commands, mut query: Query<(Entity, &mut TTL)>)
{
    for (entity, mut ttl) in &mut query
    {
        if ttl.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
        ttl.0 -= time.delta_secs();
    }
}

pub fn apply_gravity(mut query: Query<(&Mass, &mut Forces), With<Gravity>>)
{
    for (mass, mut forces) in &mut query
    {
        let gravity_force = Force(Vec3::new(0.0, -9.81 * mass.0, 0.0));
        forces.0.push(gravity_force);
    }
}