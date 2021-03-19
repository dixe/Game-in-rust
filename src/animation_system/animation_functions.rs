use crate::entity;

pub fn idle_bob_z(time_passed: f32, physics: &mut entity::Physics) {

    physics.pos.z = (time_passed * std::f32::consts::PI).sin();

}


pub fn spin_around(time_passed: f32, physics: &mut entity::Physics) {

    let angle = (time_passed * std::f32::consts::PI);
    println!("{}", physics.pos.x);
    physics.pos.x = angle.cos();
    physics.pos.y = angle.sin();

}
