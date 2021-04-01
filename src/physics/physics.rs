use crate::game;
use crate::entity;
use crate::physics::impulse_resolution::*;
use crate::physics::projection_collision::*;


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

    //NON IMPULSE COLLISION

    let weapon_col_shape = create_entity_collision_shape(ctx.player_weapon_id, ctx);

    let mut enemies = Vec::<(usize, ConvexCollisionShape)>::new();
    for enemy_id in &ctx.state.enemies {
        match create_entity_collision_shape(*enemy_id, ctx) {
            Some(col_shape) => {
                enemies.push((*enemy_id, col_shape));
            },
            None => continue
        };
    }


    // ONLY DO WHEN PLAYER IS ATTACKING
    // ALSO MAYBE MOVE TO NOT PHYSCIS, SINCE WE DON'T
    // HAVE IT HERE SINCE NO PHYSICS IS GOING ON
    // ALSO WHEN PLAYER ATTACKING IS NOT A PHYSICS CONCERN
    match weapon_col_shape {
        Some(weapon) => {
            weapon_collision(&weapon, enemies);
        },
        _ => {}
    };





    impulse_collisions

}



fn weapon_collision(weapon: &ConvexCollisionShape, enemies: Vec::<(usize, ConvexCollisionShape)>) {

    for (id, enemy) in &enemies {
        let (col, _, _) = collision_sat_shapes_impulse(weapon, &enemy);
        if col {
            println!("Hit yuo stupid");
        }
    }
}


fn update_entities_position(ctx: &mut game::Context) {
    let delta = ctx.get_delta_time();

    for physics in ctx.ecs.physics.values_mut() {
        physics.pos += physics.velocity * delta;
    }
}


fn update_entities_rotation (ctx: &mut game::Context) {
    let delta = ctx.get_delta_time();

    for physics in ctx.ecs.physics.values_mut() {
        let target_r = f32::atan2(physics.target_dir.y, physics.target_dir.x);
        let mut diff = target_r - physics.rotation.z;

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

            physics.rotation.z += rot;
        }
    }
}



fn create_entity_collision_shape(entity_id: usize, ctx: &game::Context) -> Option<ConvexCollisionShape> {
    game::get_absoulte_physics(entity_id, &ctx.ecs).map(|physics| {
        ConvexCollisionShape::rectangle(&physics.pos, 1.0, 1.0, &physics)
    })
}
