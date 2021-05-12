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
    for e1_hitbox_base in &e1.hitboxes {
        let e1_hitbox = e1_hitbox_base.make_transformed(e1.physics.pos, e1.physics.rotation);

        for e2_hitbox_base in &e1.hitboxes {
            let e2_hitbox = e2_hitbox_base.make_transformed(e2.physics.pos, e2.physics.rotation);

            let collision_res = check_collision(&e1_hitbox, &e2_hitbox);
            match collision_res {
                CollisionResult::Collision(resolve_vec) => {
                    println!("RESOLVE {:?}", resolve_vec);
                    e1.physics.pos -= resolve_vec;
                },
                _ => {}
            };


        }
    }
}


fn resolve_world_collision_entity(e1: &mut entity::Entity, world: &[Triangle] ) {

    // TODO make this more optimized, by calculation each transformed hitbox only once
    for e1_hitbox_base in &e1.hitboxes {
        let e1_hitbox = e1_hitbox_base.make_transformed(e1.physics.pos, e1.physics.rotation);

        let collision_res = check_collision_triangles(&e1_hitbox, world);

        let slope_hitbox = e1_hitbox_base.make_transformed(e1.physics.pos - na::Vector3::new(0.0, 0.0, 0.4), e1.physics.rotation);
        let collision_slope_res = check_collision_triangles(&slope_hitbox, world);

        match (collision_res, collision_slope_res) {
            (CollisionResult::Collision(resolve_vec),_) => {
                e1.physics.pos += resolve_vec;

                // also fix velocity
                let is_down = na::Vector3::new(0.0, 0.0, 1.0).dot(&resolve_vec.normalize());

                //e1.physics.pos -= na::Vector3::new(0.0, 0.0, 0.4);

                if is_down > 0.2 {
                    //e1.physics.velocity.z = 0.0;
                }
            },

            (CollisionResult::NoCollision, CollisionResult::Collision(resolve_vec)) => {

                let diff  = 0.4 - resolve_vec.z;
                e1.physics.pos.z -= diff;
                println!(" floting by stil touching MAG {} VEC {:?}", diff, resolve_vec);

            },
            (a,b) => {
                println!("a: {:?}\nb: {:?}", a, b);

            }
        };
    }
}
