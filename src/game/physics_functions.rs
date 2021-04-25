use nalgebra as na;

use crate::entity;


pub fn update_velocity(entity: &mut entity::Physics, velocity_change: na::Vector3::<f32>,)  {

    if velocity_change.x == 0.0 && velocity_change.y == 0.0 && velocity_change.z == 0.0 {
        entity.velocity = na::Vector3::new(0.0, 0.0, 0.0);
        return;
    }

    entity.velocity = velocity_change + entity.velocity;

    let speed = entity.velocity.magnitude();

    // avoid jittering
    if speed < 0.1 {
        entity.velocity *= 0.0;
    }

    if speed > entity.max_speed {
        entity.velocity *= entity.max_speed / speed;
    }
}


pub fn update_rotation(entity: &mut entity::Physics, look_dir: na::Vector3::<f32>,)  {

    // TODO maybe have a cap on rotation. so entities cannot turn around in 1 frame, but need like X frame for 1 full rotation
    // Just define a rotation_speed on entity

    // TODO kinda broken with physics, maybe the normals are getting wrong, or we rotate wrong direction? right vs left hand coordinate system ect.

    let rotation = get_rotation(&look_dir);
    let z_angle = f32::atan2(rotation.sin, rotation.cos);

    let z_angle_diff = z_angle - entity.rotation.z;

    // z_angle
    entity.rotation.z += z_angle_diff;
}


pub fn update_velocity_and_rotation(entity: &mut entity::Physics, velocity_change: na::Vector3::<f32>,)  {
    update_velocity(entity, velocity_change);
    update_rotation(entity, entity.velocity);
}


pub fn get_absoulte_physics(entity_id: usize, entities: &entity::Entities) -> Option<entity::Physics> {
    match entities.get(entity_id) {
        None => None,
        Some(e) => Some(e.physics),
    }

}



#[derive(Debug)]
pub struct Rotation {

    pub sin: f32,
    pub cos: f32
}


//TODO maybe just use atan2 instead
pub fn get_rotation(dir: &na::Vector3<f32>) -> Rotation {
    let mut cos =  1.0;
    let mut sin = 0.0;

    if dir.x != 0.0 || dir.y != 0.0 {

        cos = na::Vector3::new(1.0, 0.0, 0.0).dot(&dir.normalize());
        let rotation_sin_vec = na::Vector3::new(1.0, 0.0, 0.0).cross(&dir.normalize());
        sin = rotation_sin_vec.z.signum() * rotation_sin_vec.magnitude();
    }

    Rotation {
        sin,
        cos
    }
}
