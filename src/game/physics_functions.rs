use nalgebra as na;

use crate::physics;
use crate::shot;
use crate::entity;



pub fn update_velocity_and_rotation(entity: &mut entity::Physics, new_dir: na::Vector3::<f32>,)  {

    if new_dir.x == 0.0 && new_dir.y == 0.0 && new_dir.z == 0.0 {
        entity.velocity = na::Vector3::new(0.0, 0.0, 0.0);
        return;
    }

    let mut new_vel = new_dir.normalize() * entity.acceleration + entity.velocity;

    let speed = new_vel.magnitude();

    if speed > entity.max_speed {
        new_vel *= entity.max_speed / speed;
    }


    //TODO maybe have a cap on rotation. so entities cannot turn around in 1 frame, but need like X frame for 1 full rotation
    // Just define a rotation_speed on entity
    let rotation = get_rotation(&new_vel);

    entity.rotation_sin = rotation.sin;
    entity.rotation_cos = rotation.cos;


    entity.velocity =  new_vel;
}



pub struct Rotation {

    pub sin: f32,
    pub cos: f32
}

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
