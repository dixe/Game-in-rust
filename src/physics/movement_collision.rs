use crate::entity;
use crate::game;
use crate::physics::collision_3d::*;

pub fn resolve_movement_collision(ctx: &mut game::Context) {

    for enemy in ctx.entities.enemies.entities.values_mut() {
        resolve_movement_collision_entities(&mut ctx.entities.player, enemy);
    }

    resolve_world_collision_entity(&mut ctx.entities.player, &ctx.world_triangles);


}


fn resolve_movement_collision_entities(e1: &mut entity::Entity, e2: &mut entity::Entity ) {

    // TODO make this more optimized, by calculation each transformed hitbox only once
    for e1_hitbox_base in &e1.hit_boxes {
        let e1_hitbox = e1_hitbox_base.make_transformed(e1.physics.pos, e1.physics.rotation);

        for e2_hitbox_base in &e1.hit_boxes {
            let e2_hitbox = e2_hitbox_base.make_transformed(e2.physics.pos, e2.physics.rotation);

            let collision_res = check_collision(&e1_hitbox, &e2_hitbox);
            match collision_res {
                CollisionResult::Collision(resolve_vec) => {
                    e1.physics.pos -= resolve_vec;
                },
                _ => {}
            };


        }
    }
}


fn resolve_world_collision_entity(e1: &mut entity::Entity, world: &[Triangle] ) {

    // TODO make this more optimized, by calculation each transformed hitbox only once
    for e1_hitbox_base in &e1.hit_boxes {
        let e1_hitbox = e1_hitbox_base.make_transformed(e1.physics.pos, e1.physics.rotation);

        let collision_res = check_collision_triangles(&e1_hitbox, world);

        match collision_res {
            CollisionResult::Collision(resolve_vec) => {
                //println!("{:?}", resolve_vec);
                e1.physics.pos += resolve_vec;
            },
            _ => {}
        };
    }
}
