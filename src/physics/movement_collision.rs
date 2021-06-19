use crate::entity;
use crate::game;
use crate::types::*;
use crate::physics::collision_3d::*;

use quadtree as qt;

pub fn resolve_movement_collision(scene: &mut game::Scene) {

    for enemy in scene.entities.enemies.entities.values_mut() {
        resolve_movement_collision_entities(&mut scene.entities.player, enemy);
    }

    let mut triangle_indices = scene.world_triangles_tree.query(&qt::Query::point(scene.entities.player.physics.pos.x as i32, scene.entities.player.physics.pos.y as i32));

    triangle_indices.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // maybe use a free list stored on scene or something to avoid reallocating each frame
    // or just give the function both a list of triangles and list of indices??
    let mut triangles = Vec::new();
    for i in &triangle_indices {
        triangles.push(scene.world_triangles[**i]);
    }

    //println!(" ({}, {}) {:?} ", scene.entities.player.physics.pos.x as i32, scene.entities.player.physics.pos.y as i32, triangle_indices);

    resolve_world_collision_entity(&mut scene.entities.player, &triangles);


    for enemy in scene.entities.enemies.values_mut() {

        let mut triangle_indices = scene.world_triangles_tree.query(&qt::Query::point(enemy.physics.pos.x as i32, enemy.physics.pos.y as i32));

        triangle_indices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // maybe use a free list stored on scene or something to avoid reallocating each frame
        // or just give the function both a list of triangles and list of indices??
        let mut triangles = Vec::new();
        for i in &triangle_indices {
            triangles.push(scene.world_triangles[**i]);
        }

        resolve_world_collision_entity(enemy, &triangles);
    }

}


fn resolve_movement_collision_entities(entity: &mut entity::Entity, e2: &mut entity::Entity ) {

    // TODO make this more optimized, by calculation each transformed hitbox only once
    for entity_hitbox_base in &entity.hitboxes {
        let entity_hitbox = entity_hitbox_base.make_transformed(entity.physics.pos, entity.physics.rotation);

        for e2_hitbox_base in &entity.hitboxes {
            let e2_hitbox = e2_hitbox_base.make_transformed(e2.physics.pos, e2.physics.rotation);

            let collision_res = check_collision(&entity_hitbox, &e2_hitbox);
            match collision_res {
                CollisionResult::Collision(resolve_vec) => {
                    entity.physics.pos -= resolve_vec;
                },
                _ => {}
            };


        }
    }
}


fn resolve_world_collision_entity(entity: &mut entity::Entity, world: &[Triangle] ) {

    // TODO make this more optimized, by calculation each transformed hitbox only once
    for entity_hitbox_base in &entity.hitboxes {
        let entity_hitbox = entity_hitbox_base.make_transformed(entity.physics.pos, entity.physics.rotation);

        let collision_res = check_collision_triangles(&entity_hitbox, world);

        match collision_res {
            CollisionResult::Collision(resolve_vec) => {
                let resolve_threshold = 0.001;
                //println!("regular resolve VEC {:?}", resolve_vec);

                // to avoid jitter ing between this regular resolve and the floating by still touching
                // state. This jitter is due to float numeric instability/imperfections/precision limit
                // This will keep entity in this state

                if resolve_vec.x.abs() > resolve_threshold {
                    entity.physics.pos.x += resolve_vec.x;
                }
                if resolve_vec.y.abs() > resolve_threshold {
                    entity.physics.pos.y += resolve_vec.y
                }

                entity.physics.pos.z += resolve_vec.z;

                entity.physics.falling = false;
                entity.physics.velocity.z = 0.0;
            },
            _ => {



                let mut set_falling = true;
                // If not falling and close to ground, snap to ground, to avoid jitter
                if !entity.physics.falling {
                    let p = entity.physics.pos;

                    let mut max_z = 0.0;
                    for triangle in world {
                        let projection  = triangle.project_point_z_axis(&p);

                        // close to gorund maybe don't fall
                        if projection.z < 0.01 {
                            let inside = triangle.inside(&projection);

                            if inside {
                                set_falling = false;
                                // to avoid jitter don't snap when too close to ground
                                if projection.z > 0.001 {
                                    max_z = f32::max(max_z, projection.z);
                                }
                            }
                        }
                    }

                    if !set_falling {
                        entity.physics.pos.z -= max_z;
                    }

                }

                entity.physics.falling = set_falling;
            }
        };
    }
}
