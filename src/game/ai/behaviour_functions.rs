use nalgebra as na;
use crate::entity::{Entity, EntityState}; use crate::game;

pub fn move_to_point(entity: &mut Entity, new_point: na::Vector3<f32>) {

    //TODO: Maybe remove the Z component, since for movement it is not used.
    // Unlesss entity is falling and we need to wait until it has fallen?
    // Error is that when stading on ground we are not a z = 0.0, but higer.
    // maybe also remove the rotation aspect, since walking sideways and backwards

    let mut move_vec = new_point - entity.physics.pos;

    move_vec.z = 0.0;

    let move_mag = move_vec.magnitude();

    if move_mag > 0.1 {
        move_vec = move_vec.normalize();

        move_vec = move_vec * entity.physics.max_speed;
    }
    else {
        move_vec = move_vec.normalize();
    }

    game::set_velocity(&mut entity.physics, move_vec);
    entity.physics.facing_dir =  move_vec.normalize();

}
