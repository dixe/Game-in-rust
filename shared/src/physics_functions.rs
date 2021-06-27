use nalgebra as na;

use crate::base_entity;


pub fn update_velocity(entity: &mut base_entity::Physics, vel_change: na::Vector3::<f32>,)  {

    if vel_change.x == 0.0 && vel_change.y == 0.0 && vel_change.z == 0.0 {
        entity.velocity = na::Vector3::new(0.0, 0.0, entity.velocity.z);
        return;
    }

    entity.velocity += vel_change;

    let speed = entity.velocity.magnitude();

    // avoid jittering
    if speed < 0.1 {
        entity.velocity.x = 0.0;
        entity.velocity.y = 0.0;
    }

    if speed > entity.max_speed {
        entity.velocity.x *= entity.max_speed / speed;
        entity.velocity.y *= entity.max_speed / speed;
    }

}


pub fn set_velocity(entity: &mut base_entity::Physics, new_velocity: na::Vector3::<f32>,)  {

    if new_velocity.x == 0.0 && new_velocity.y == 0.0 && new_velocity.z == 0.0 {
        entity.velocity = na::Vector3::new(0.0, 0.0, 0.0);
        return;
    }

    entity.velocity = new_velocity;


    let speed = entity.velocity.magnitude();

    if speed > entity.max_speed {
        entity.velocity.x *= entity.max_speed / speed;
        entity.velocity.y *= entity.max_speed / speed;
    }

}
