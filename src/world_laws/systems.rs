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
        let gravity_force = Force(Vec3::new(0.0, -98.1 * mass.0, 0.0));
        forces.0.push(gravity_force);
    }
}

pub fn collision_system(query: Query<(Entity, &ColliderShape)>, mut forces_query: Query<&mut Forces>,) // powered by Separating Axis Theorem (SAT) and Minimum Translation Vector (MTV)
{
    for [(entity_a, collider_a),
         (entity_b, collider_b)]
        in query.iter_combinations()
    {
        let collide_vec = 
            measure_collision_between_shapes(
                collider_a,
                collider_b,
            ).extend(0.0);

        if let Ok(mut forces_a) = forces_query.get_mut(entity_a) {
            forces_a.0.push(Force(collide_vec));
        }

        if let Ok(mut forces_b) = forces_query.get_mut(entity_b) {
            forces_b.0.push(Force(-collide_vec));
        }
    }

    fn get_axises_from_shape(shape: &ColliderShape) -> Vec<Vec2>
    {
        let mut uniq_axises: Vec<Vec2> = Vec::new();
        let mut points_to_check = shape.points.clone();
        points_to_check.push(shape.points[0]);

        for pair in points_to_check.windows(2)
        {
            let axis = (pair[0] - pair[1]).normalize();
            if !uniq_axises.contains(&axis)
            {
                uniq_axises.push(axis);
            }
        }
        uniq_axises
    }

    fn project_point_on_axis(point: &Vec2, axis_start: &Vec2, axis_vec: &Vec2) -> Vec2
    {
        let vec = point - axis_start;
        let scalar_projection = (vec * axis_vec) / axis_vec.length_squared();
        let projected_point = axis_start + scalar_projection * axis_vec;
        projected_point
    }

    fn project_shape_on_axis(shape: &ColliderShape, axis_start: &Vec2, axis_vec: &Vec2) -> (f32, f32)
    {
        let mut min = f32::MAX;
        let mut max = f32::MIN;

        for point in &shape.points
        {
            let projected_point = project_point_on_axis(&point, &axis_start, &axis_vec);
            let projection_length = projected_point.length();

            if projection_length < min
            {
                min = projection_length;
            }
            if projection_length > max
            {
                max = projection_length;
            }
        }
        (min, max)
    }

    fn measure_collision_between_shapes(shape1: &ColliderShape, shape2: &ColliderShape) -> Vec2 // TODO: make readable
    {
        let mut overlap: f32 = f32::MAX;
        let mut axises1: Vec<Vec2> = Vec::new();
        let mut axises2: Vec<Vec2> = Vec::new();
        for axis in get_axises_from_shape(&shape1) // gets normals from shape sides 
        {
            let normal = Vec2::new(-axis.y, axis.x).normalize();
            axises1.push(normal);
        }
        for axis in get_axises_from_shape(&shape2)
        {
            let normal = Vec2::new(-axis.y, axis.x).normalize();
            axises2.push(normal);
        }

        let mut smallest = axises1[0];
        for axis in axises1.iter().chain(axises2.iter())
        {
            let (min1, max1) = project_shape_on_axis(&shape1, &Vec2::ZERO, &axis);
            let (min2, max2) = project_shape_on_axis(&shape2, &Vec2::ZERO, &axis);

            if max1 < min2 || max2 < min1 // check if overlap (if true - no overlap)
            {
                return Vec2::ZERO;
            }
            else // calculate penetration depth
            {
                let o = (max2 - min1).min(max1 - min2); // overlap length
                if o < overlap
                {
                    overlap = o;
                    smallest = *axis;
                }
            }
        }
        return smallest.normalize() * overlap;
    }
}