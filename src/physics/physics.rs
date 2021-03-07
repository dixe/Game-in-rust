use nalgebra as na;


use crate::scene;
use crate::entity;
use crate::game;

use crate::physics::projection_collision::{collision_sat, CollisionBox, generate_sides, generate_vertices, collision_side};


pub struct Collisions {
    pub enemies_hit: Vec<Hit>

}

pub struct Hit {

    pub entity_id: usize,
    pub projectile_id: usize,

}


pub fn process(ctx: &mut game::Context) -> Collisions {

    let mut collisions = Collisions {
        enemies_hit: Vec::<Hit>::new(),
    };

    let delta = ctx.get_delta_time();

    let mut player = match ctx.entity_manager.get_entity(ctx.player_id) {
        Some(p) => p,
        None => return collisions
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

        for proj in &mut ctx.player_projectiles {
            let mut p = match ctx.entity_manager.get_entity(proj.entity_id) {
                Some(e) => e,
                None => continue
            };

            // println!("{}", p.velocity);
            p.set_position(p.pos + p.velocity*delta);
            ctx.entity_manager.update_entity(p);

            if entities_collide(&p, &e) {
                collisions.enemies_hit.push(Hit { entity_id: e.id, projectile_id: e.id});
            }

        }


        if entities_collide(&player, &e) {
            println!("OUCH");
        }


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


    ctx.entity_manager.update_entity(player);

    collisions
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

    let (col, _) = collision_sat(generate_vertices(&entity_1_col_box), generate_sides(&entity_2_col_box).as_slice());

    col
}


fn entity_update_movement(entity: &mut entity::Entity, delta: f32, movement_dir: &na::Vector3::<f32>, scene: &scene::Scene) {

    let new_entity_velocity = new_velocity(&movement_dir, &entity.velocity, entity.acceleration, entity.max_speed);

    let mut entity_pos_updated = entity.pos + new_entity_velocity * delta;




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
    entity.set_velocity(new_entity_velocity);

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
