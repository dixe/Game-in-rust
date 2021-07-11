use shared::*;
use nalgebra as na;

#[derive(PartialEq)]
pub enum KeepDistanceResult {
    InDistance,
    MovingAway,
    MovingTowards
}


pub fn keep_distance(distance: f32, entity: &mut BaseEntity, target: &BaseEntity) -> KeepDistanceResult {

    let target_dist = (entity.physics.pos - target.physics.pos).magnitude();
    let mut vel = entity.physics.pos - target.physics.pos;

    vel.z = 0.0;
    let leway = distance * 0.1;


    entity.physics.facing_dir = (-vel).normalize();

    if target_dist > (distance - leway) && target_dist < (distance + leway) {
        physics_functions::set_velocity(&mut entity.physics, na::Vector3::new(0.0, 0.0,0.0));
        return KeepDistanceResult::InDistance;
    }

    if target_dist < (distance - leway) {
        physics_functions::set_velocity(&mut entity.physics, vel );
        return KeepDistanceResult::MovingAway;
    }

    physics_functions::set_velocity(&mut entity.physics, -vel);

    KeepDistanceResult::MovingTowards
}
