use crate::game;
use crate::entity;

pub fn idle_bob_z(time_passed: f32, physics: &mut entity::Physics, _ : &entity::Physics) {

    physics.pos.z = (time_passed * std::f32::consts::PI).sin();
}

pub fn empty(_time_passed: f32, _physics: &mut entity::Physics, _ : &entity::Physics) {
}


pub fn set_player_moving(state: &mut game::State) {
    state.player_state = game::PlayerState::Moving;
}



pub fn bezier_linear(t: f32, p0: na::Vector3<f32>, p1: na::Vector3<f32>) -> na::Vector3<f32> {
    (1.0 - t) * p0 + t * p1
}

pub fn bezier_cubic(t: f32, p0: na::Vector3<f32>, p1: na::Vector3<f32>, p2: na::Vector3<f32>) -> na::Vector3<f32> {
    p1 + (1.0 - t)* (1.0 - t) * (p0 - p1) + t*t * (p2-p1)
}
