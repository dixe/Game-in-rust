use crate::game;

use crate::physics::impulse_resolution::*;
use crate::physics::projection_collision::*;
use crate::physics::movement_collision::*;


#[derive(Copy, Clone, Debug)]
pub struct EntityCollision {
    pub entity_1_id: usize,
    pub entity_2_id: usize
}


pub fn process(ctx: &mut game::Context) -> Vec<EntityCollision> {
    // MOVE ENTITIES

    update_entities_position(ctx);
    update_entities_rotation(ctx);


    //DO IMPULSE COLLISION AND UPDATE
    let impulse_collisions = do_impulse_correction(ctx);


    resolve_movement_collision(ctx);



    // TODO remove
    impulse_collisions

}



fn update_entities_position(ctx: &mut game::Context) {
    // Should this maybe be done more explicity
    // Maybe with a list of physics entities or something like that, or just go over all of them
    // and take the ones where we want physics
    let delta = ctx.get_delta_time();


    //for entity in ctx.entities.values_mut() {
    let entity  = &mut ctx.entities.player;
    // maybe there is root_motion
    let mut update_with_vel = true;

    match entity.animation_player.as_mut() {
        Some(animation_player) => {
            match animation_player.current_root_motion() {
                Some(root_motion) =>
                {
                    update_with_vel = false;

                    // This should not be camera or controls dependent as it will also need to work for
                    // enemies and other non player entities.
                    // Maybe have a root_motion_rotation on entity, that can be set but entity
                    // Or animaiton player, for the relavant animaiton
                    let z_rot = entity.physics.rotation.euler_angles().2;

                    let rot_mat = na::Matrix3::new_rotation(z_rot);
                    let offset = rot_mat * root_motion;


                    // maybe still apply z for gravity
                    entity.physics.pos += offset;
                },
                None => {}
            }
        },
        _ => {}
    }

    if update_with_vel {
        entity.physics.pos += entity.physics.velocity * delta;
    }

    // APPLY GRAVITY -- results in jancky motion down hill
    let v = entity.physics.velocity.z;

    let a = -400.0;
    let gravity = v * delta + (1.0/2.0 * a * delta * delta);

    //entity.physics.velocity.z += gravity;
}


fn update_entities_rotation (ctx: &mut game::Context) {
    let delta = ctx.get_delta_time();


    for entity in ctx.entities.values_mut() {

        let mut physics = entity.physics;
        let target_r = f32::atan2(physics.target_dir.y, physics.target_dir.x);

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

        entity.physics = physics;
    }
}
