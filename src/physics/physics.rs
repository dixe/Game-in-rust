use nalgebra as na;


use crate::scene;
use crate::entity;

use crate::game;

use crate::physics::projection_collision::{collision_sat, CollisionBox};


pub fn process(ctx: &mut game::Context) {

    entity_update_movement(&mut ctx.player, &ctx.controls.movement_dir, &ctx.scene);

    for e in &mut ctx.enemies {
        let move_dir = na::Vector3::new(1.1,0.2,0.0).normalize();

        entity_update_movement(e, &move_dir, &ctx.scene);

        if entities_collide(&ctx.player, e) {
            println!("OUCH");
        }
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

    let (col, _) = collision_sat(&entity_1_col_box, &entity_2_col_box);

    col
}


fn entity_update_movement(entity: &mut entity::Entity, movement_dir: &na::Vector3::<f32>, scene: &scene::Scene) {
    let mut new_entity_velocity = new_velocity(&movement_dir, &entity.velocity, entity.acceleration, entity.max_speed);

    let mut entity_pos_updated = entity.pos + new_entity_velocity;

    for wall_pos in &scene.border_positions {

        let wall_collision_box =  CollisionBox {
            pos: *wall_pos,
            side_len: 1.0,
        };

        let entity_col_box = CollisionBox {
            pos: entity_pos_updated,
            side_len: 1.0,
        };


        let (col, dir) = collision_sat(&entity_col_box, &wall_collision_box);

        if col {

            entity_pos_updated -= dir;
            new_entity_velocity -= dir;
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
