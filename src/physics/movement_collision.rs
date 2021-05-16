use crate::entity;
use crate::game;
use crate::physics::collision_3d::*;

pub fn resolve_movement_collision(scene: &mut game::Scene) {

    for enemy in scene.entities.enemies.entities.values_mut() {
        resolve_movement_collision_entities(&mut scene.entities.player, enemy);
    }

    resolve_world_collision_entity(&mut scene.entities.player, &scene.world_triangles);

    for enemy in scene.entities.enemies.values_mut() {
        resolve_world_collision_entity(enemy, &scene.world_triangles);
    }

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
                let resolve_threshold = 0.001;
                //println!("regular resolve VEC {:?}", resolve_vec);

                // to avoid jitter ing between this regular resolve and the floating by still touching
                // state. This jitter is due to float numeric instability/imperfections/precision limit
                // This will keep entity in this state

                if resolve_vec.x.abs() > resolve_threshold {
                    e1.physics.pos.x += resolve_vec.x;
                }
                if resolve_vec.y.abs() > resolve_threshold {
                    e1.physics.pos.y += resolve_vec.y
                }



                // Close to 0 means steeper wall
                let angle_dot = na::Vector3::new(0.0, 0.0, 1.0).dot(&resolve_vec.normalize());
                //println!("angle_dot {:?}", angle_dot);
                // 0.8 is about 64 degrees, acos(0.8) = 0.64 rad = 36 deg. 90-34 = 54
                // or asin(0.8) = 92 rad = 54 deg
                if resolve_vec.z.abs() > resolve_threshold && (angle_dot > 0.8 || angle_dot < 0.0) {
                    e1.physics.pos.z += resolve_vec.z;
                }



                e1.physics.falling = true;
                e1.physics.velocity.z = 0.0;
            },

            (CollisionResult::NoCollision, CollisionResult::Collision(resolve_vec)) => {

                let diff  = 0.4 - resolve_vec.z;
                //if diff > 0.1 {
                e1.physics.pos.z -= diff;
                //                }
                //println!("floting by stil touching MAG {} VEC {:?}", diff, resolve_vec);
                e1.physics.falling = false;
                e1.physics.velocity.z = 0.0;

            },

            _ => {
                e1.physics.falling = true;
            }
        };
    }
}
