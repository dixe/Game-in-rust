use crate::entity;

pub fn idle_bob_z(time_passed: f32, physics: &mut entity::Physics) {
    physics.pos.z = (time_passed / 300.0).sin();
}
