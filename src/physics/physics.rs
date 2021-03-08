use nalgebra as na;


use crate::scene;
use crate::entity;
use crate::game;

use crate::physics::projection_collision::{collision_sat, CollisionBox, generate_side_from_bb, generate_vertices, collision_side};


pub struct Collisions {
    pub enemies_hit: Vec<Hit>,
    pub player_enemies_collision: Vec<usize>,


}



pub struct Hit {

    pub entity_id: usize,
    pub projectile_id: usize,

}


pub fn process(ctx: &mut game::Context) -> Collisions {

    let mut collisions = Collisions {
        enemies_hit: Vec::<Hit>::new(),
        player_enemies_collision: Vec::<usize>::new(),
    };

    let delta = ctx.get_delta_time();

    let mut player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(p) => p,
        None => return collisions
    };

    entity_update_movement_scene(&mut player, delta, &ctx.scene);

    for e_id in &mut ctx.enemies {
        let mut e = match ctx.ecs.get_physics(*e_id) {
            Some(en) => en,
            None => continue
        };

        entity_update_movement_scene(&mut e, delta, &ctx.scene);

        for proj in &mut ctx.player_projectiles {
            let mut p = match ctx.ecs.get_physics(proj.entity_id) {
                Some(e) => e,
                None => continue
            };


            p.set_position(p.pos + p.velocity*delta);
            ctx.ecs.set_physics(proj.entity_id, p);

            let (col, _) = entities_collide(&p, &e);
            if col {
                collisions.enemies_hit.push(Hit { entity_id: *e_id, projectile_id: proj.entity_id});
            }

        }




        let (col, dir) = entities_collide(&player, &e);

        if col {
            e.pos -= dir;
            collisions.player_enemies_collision.push(*e_id);
        }

        ctx.ecs.set_physics(*e_id, e);
    }


    // handle
    for proj in &mut ctx.player_projectiles {
        let mut p = match ctx.ecs.get_physics(proj.entity_id) {
            Some(e) => e,
            None => continue
        };

        // println!("{}", p.velocity);
        p.set_position(p.pos + p.velocity*delta);
        ctx.ecs.set_physics(proj.entity_id, p);


    }


    ctx.ecs.set_physics(ctx.player_id, player);

    collisions
}


fn entities_collide(entity_1: &entity::Physics, entity_2: &entity::Physics) -> (bool, na::Vector3::<f32>) {

    let entity_1_col_box = CollisionBox {
        pos: entity_1.pos,
        side_len: 1.0,
    };

    let entity_2_col_box = CollisionBox {
        pos: entity_2.pos,
        side_len: 1.0,
    };

    collision_sat(generate_vertices(&entity_1_col_box), generate_side_from_bb(&entity_2_col_box).as_slice())
}


fn entity_update_movement_scene(entity: &mut entity::Physics, delta: f32, scene: &scene::Scene) {

    let mut entity_pos_updated = entity.pos + entity.velocity * delta;

    for wall_side in scene.border_sides() {
        let entity_col_box = CollisionBox {
            pos: entity_pos_updated,
            side_len: 1.0,
        };

        let (col, dir) = collision_side(generate_vertices(&entity_col_box), &wall_side);

        if col {
            entity_pos_updated -= dir;
        }
    }

    entity.set_position(entity_pos_updated);

}
