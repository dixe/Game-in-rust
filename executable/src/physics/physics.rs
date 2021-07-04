use crate::game;

use crate::physics::impulse_resolution::*;

use crate::physics::movement_collision::*;

use crate::entity;


#[derive(Copy, Clone, Debug)]
pub struct EntityCollision {
    pub entity_1_id: usize,
    pub entity_2_id: usize
}


pub fn process(scene: &mut game::Scene, delta: f32) -> Vec<EntityCollision> {
    // MOVE ENTITIES

    update_entities_position(scene, delta);
    update_entities_rotation(scene, delta);


    //DO IMPULSE COLLISION AND UPDATE
    let impulse_collisions = do_impulse_correction(scene);

    resolve_movement_collision(scene);

    // TODO remove
    impulse_collisions

}



fn update_entities_position(scene: &mut game::Scene, delta: f32) {
    // Should this maybe be done more explicity
    // Maybe with a list of physics entities or something like that, or just go over all of them
    // and take the ones where we want physics

    //for entity in scene.entities.values_mut() {
    let _entity  = &mut scene.entities.player;
    update_entity_position(&mut scene.entities.player, delta);

    for enemy in scene.entities.enemies.values_mut() {
        update_entity_position(enemy, delta);
    }
}


fn update_entity_position(entity: &mut entity::Entity, delta: f32) {
    // maybe there is root_motion
    let mut update_with_vel = true;

    match entity.animation_player.as_mut() {
        Some(animation_player) => {
            match animation_player.current_root_motion() {
                Some(root_motion) =>
                {
                    update_with_vel = false;
                    let z_rot = entity.base_entity.physics.rotation.euler_angles().2;

                    let rot_mat = na::Matrix3::new_rotation(z_rot);
                    let offset = rot_mat * root_motion;


                    // maybe still apply z for gravity
                    entity.base_entity.physics.pos += offset;
                },
                None => {}
            }
        },
        _ => {}
    }

    if update_with_vel {
        entity.base_entity.physics.pos += entity.base_entity.physics.velocity * delta;
    }

    if entity.base_entity.physics.falling {
        // APPLY GRAVITY -- results in jancky motion down hill
        let v = entity.base_entity.physics.velocity.z;
        let a = -9.82;
        let gravity = v + (1.0/2.0 * a);

        entity.base_entity.physics.velocity.z += gravity * delta;
    }
}


fn update_entities_rotation (scene: &mut game::Scene, delta: f32) {

    update_entity_rotation(&mut scene.entities.player, delta);

    for entity in scene.entities.enemies.values_mut() {
        update_entity_rotation(entity, delta);
    }
}



fn update_entity_rotation(entity: &mut entity::Entity, delta: f32) {

    let mut physics = entity.base_entity.physics;
    let target_r = f32::atan2(physics.facing_dir.y, physics.facing_dir.x);

    let mut diff = target_r - physics.rotation.euler_angles().2;

    if diff < -std::f32::consts::PI {
        diff += 2.0 * std::f32::consts::PI;
    }
    if diff > std::f32::consts::PI {
        diff -= 2.0 * std::f32::consts::PI;
    }

    let dir = diff.signum();

    let rotation_speed = 6.0;

    let mut rot = dir * rotation_speed * delta;

    if rot.abs() > diff.abs() {
        rot = diff;
    }

    if diff.abs() > 0.01 {
        let z_rot = na::UnitQuaternion::from_euler_angles(0.0, 0.0, rot);
        physics.rotation *=  z_rot;
    }

    entity.base_entity.physics = physics;
}
