use nalgebra as na;


use crate::scene;
use crate::entity;
use crate::game;

use crate::physics::projection_collision::{collision_sat, CollisionBox, generate_sides, generate_vertices};



pub fn process(ctx: &mut game::Context) {


    let delta = ctx.get_delta_time();

    let mut player = match ctx.entity_manager.get_entity(ctx.player_id) {
        Some(p) => p,
        None => return
    };



    entity_update_movement(&mut player, delta, &ctx.controls.movement_dir, &ctx.scene);

    //    println!("enemy_speed: {}", player.velocity);

    for e_id in &mut ctx.enemies_ids {
        let mut e = match ctx.entity_manager.get_entity(*e_id) {
            Some(en) => en,
            None => continue
        };

        let move_dir = na::Vector3::new(1.1,0.2,0.0).normalize();

        entity_update_movement(&mut e, delta, &move_dir, &ctx.scene);






        ctx.entity_manager.update_entity(e);
    }


    // handle
    for proj in &mut ctx.player_projectiles {
        let mut p = match ctx.entity_manager.get_entity(proj.entity_id) {
            Some(e) => e,
            None => continue
        };

        // println!("{}", p.velocity);
        p.set_position(p.pos + p.velocity*delta);
        ctx.entity_manager.update_entity(p);


    }



    // DO NON MUTABLE UPDATES ON ALREADY UPDATED ENTITIES

    let player_n = match ctx.entity_manager.get_entity(ctx.player_id) {
        Some(p) => p,
        None => return
    };
    for e_id in &ctx.enemies_ids {
        let e = match ctx.entity_manager.get_entity(*e_id) {
            Some(en) => en,
            None => continue
        };

        if entities_collide(&player_n, &e) {
            println!("OUCH");
        }


        /*
        for p in &active_player_shots {
        if entities_collide(&p.entity, e) {
        println!("Got you");
        ^        }
    }


         */
        /*
        let player = match ctx.entity_manager.get_entity(ctx.player_id) {
        Some(p) => p,
        None => return
    };
         */



        /* for p in &mut  ctx.player_projectiles {
        if p.expired {
        continue;
    }
        p.entity.set_position(p.entity.pos + p.entity.velocity);
    }
         */

        //TODO this should just be hanldes in entityManager
        /*
        let active_player_shots: Vec<&shot::Shot> = ctx.player_projectiles.iter()
        .filter_map(|p| match p.expired {
        false => Some(p),
        true => None
    }).collect();
         */



        ctx.entity_manager.update_entity(player);

    }

}


fn entities_collide(entity_1: &entity::Entity, entity_2: &entity::Entity) -> bool {

    let entity_1_col_box = CollisionBox {
        pos: entity_1.pos,
        side_len: 1.0,
    };

    let entity_2_col_box = CollisionBox {
        pos: entity_2.pos,
        side_len: 1.0,
    };

    let (col, _) = collision_sat(generate_vertices(&entity_1_col_box), generate_sides(&entity_2_col_box));

    col
}


fn entity_update_movement(entity: &mut entity::Entity, delta: f32, movement_dir: &na::Vector3::<f32>, scene: &scene::Scene) {

    let new_entity_velocity = new_velocity(&movement_dir, &entity.velocity, entity.acceleration, entity.max_speed);

    let mut entity_pos_updated = entity.pos + new_entity_velocity * delta;

    println!("enter");
    for wall_pos in &scene.border_positions {

        let wall_collision_box =  CollisionBox {
            pos: *wall_pos,
            side_len: 1.0,
        };

        let entity_col_box = CollisionBox {
            pos: entity_pos_updated,
            side_len: 1.0,
        };


        let (col, dir) = collision_sat(generate_vertices(&entity_col_box), generate_sides(&wall_collision_box));

        if col {
            if dir.x > 0.0 {
                //let (col2, dir2) = collision_sat(&entity_col_box, &wall_collision_box);
            }
            println!("col_dir: {}", dir);
            entity_pos_updated -= dir;

        }
    }

    entity.set_position(entity_pos_updated);
    entity.set_velocity(new_entity_velocity);

}


#[derive(Debug)]
struct Collision {
    col: bool,
    x_col: bool,
    y_col: bool
}



fn new_velocity(dir: &na::Vector3::<f32>, old_velocity: &na::Vector3::<f32>, acceleration: f32, max_speed: f32) -> na::Vector3::<f32> {

    if dir.x == 0.0 && dir.y == 0.0 && dir.z == 0.0 {
        return na::Vector3::new(0.0, 0.0, 0.0);
    }

    let mut new_vel = dir.normalize() * acceleration + old_velocity;

    let speed = new_vel.magnitude();

    if speed > max_speed {
        new_vel *= max_speed / speed;
    }

    new_vel
}
