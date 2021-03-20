use crate::entity;

pub fn idle_bob_z(time_passed: f32, physics: &mut entity::Physics, _ : &entity::Physics) {

    physics.pos.z = (time_passed * std::f32::consts::PI).sin();

}


pub fn spin_around(time_passed: f32, physics: &mut entity::Physics, init: &entity::Physics) {
    let angle_init = f32::atan2(init.pos.y, init.pos.x);

    let dir = (angle_init + std:: f32::consts::PI/2.0).signum();

    let angle = angle_init + dir * (time_passed * std::f32::consts::PI);

    physics.pos.x = angle.cos();
    physics.pos.y = angle.sin();

}
