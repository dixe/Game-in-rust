
use crate::entity;

pub fn idle_bob_z(time_passed: f32, physics: &mut entity::Physics, _ : &entity::Physics) {

    physics.pos.z = (time_passed * std::f32::consts::PI).sin();
}



pub fn bezier_linear(t: f32, p0: na::Vector3<f32>, p1: na::Vector3<f32>) -> na::Vector3<f32> {
    (1.0 - t) * p0 + t * p1
}

pub fn bezier_cubic(t: f32, p0: na::Vector3<f32>, p1: na::Vector3<f32>, p2: na::Vector3<f32>) -> na::Vector3<f32> {
    p1 + (1.0 - t)* (1.0 - t) * (p0 - p1) + t*t * (p2-p1)
}
