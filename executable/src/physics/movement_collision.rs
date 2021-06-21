use crate::entity;
use crate::game;
use crate::types::*;
use crate::physics::collision_3d::*;
use crate::types::*;
use crate::shared;

use quadtree as qt;

pub fn resolve_movement_collision(scene: &mut game::Scene) {

    for enemy in scene.entities.enemies.entities.values_mut() {
        resolve_movement_collision_entities(&mut scene.entities.player, enemy);
    }

    let mut triangle_indices = scene.world_triangles_tree.query(&qt::Query::point(scene.entities.player.base_entity.physics.pos.x as i32, scene.entities.player.base_entity.physics.pos.y as i32));

    triangle_indices.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // maybe use a free list stored on scene or something to avoid reallocating each frame
    // or just give the function both a list of triangles and list of indices??
    let mut triangles = Vec::new();
    for i in &triangle_indices {
        triangles.push(scene.world_triangles[**i]);
    }

    //println!(" ({}, {}) {:?} ", scene.entities.player.base_entity.physics.pos.x as i32, scene.entities.player.base_entity.physics.pos.y as i32, triangle_indices);

    resolve_world_collision_entity(&mut scene.entities.player, &triangles);


    for enemy in scene.entities.enemies.values_mut() {

        let mut triangle_indices = scene.world_triangles_tree.query(&qt::Query::point(enemy.base_entity.physics.pos.x as i32, enemy.base_entity.physics.pos.y as i32));

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
        let entity_hitbox = entity_hitbox_base.make_transformed(entity.base_entity.physics.pos, entity.base_entity.physics.rotation);

        for e2_hitbox_base in &entity.hitboxes {
            let e2_hitbox = e2_hitbox_base.make_transformed(e2.base_entity.physics.pos, e2.base_entity.physics.rotation);

            let collision_res = check_collision(&entity_hitbox, &e2_hitbox);
            match collision_res {
                CollisionResult::Collision(resolve_vec) => {
                    entity.base_entity.physics.pos -= resolve_vec;
                },
                _ => {}
            };


        }
    }
}


fn resolve_world_collision_entity(entity: &mut entity::Entity, world: &[Triangle] ) {

    // TODO make this more optimized, by calculation each transformed hitbox only once
    for entity_hitbox_base in &entity.hitboxes {
        let entity_hitbox = entity_hitbox_base.make_transformed(entity.base_entity.physics.pos, entity.base_entity.physics.rotation);

        let collision_res = check_collision_triangles(&entity_hitbox, world);

        match collision_res {
            CollisionResult::Collision(resolve_vec) => {
                let resolve_threshold = 0.001;
                //println!("regular resolve VEC {:?}", resolve_vec);

                // to avoid jitter ing between this regular resolve and the floating by still touching
                // state. This jitter is due to float numeric instability/imperfections/precision limit
                // This will keep entity in this state

                if resolve_vec.x.abs() > resolve_threshold {
                    entity.base_entity.physics.pos.x += resolve_vec.x;
                }
                if resolve_vec.y.abs() > resolve_threshold {
                    entity.base_entity.physics.pos.y += resolve_vec.y
                }

                if resolve_vec.z.abs() > resolve_threshold {
                    entity.base_entity.physics.pos.z += resolve_vec.z;
                }

                entity.base_entity.physics.falling = false;
                entity.base_entity.physics.velocity.z = 0.0;
            },
            _ => {

                // If not falling and close to ground, snap to ground, to avoid jitter
                if !entity.base_entity.physics.falling {

                    // TODO only have 1 movement hitbox for each entity
                    for hitbox in &entity.hitboxes {

                        let max_x = hitbox.max_x();
                        let max_y = hitbox.max_y();
                        let min_x = hitbox.min_x();
                        let min_y = hitbox.min_y();


                        let mut fall_or_slide_res = FallOrSlide::Fall;

                        fall_or_slide(&(entity.base_entity.physics.pos + v3::new(max_x, max_y,0.0)), &world, &mut fall_or_slide_res);
                        fall_or_slide(&(entity.base_entity.physics.pos + v3::new(min_x, max_y,0.0)), &world, &mut fall_or_slide_res);
                        fall_or_slide(&(entity.base_entity.physics.pos + v3::new(max_x, min_y,0.0)), &world, &mut fall_or_slide_res);
                        fall_or_slide(&(entity.base_entity.physics.pos + v3::new(min_x, min_y,0.0)), &world, &mut fall_or_slide_res);

                        match fall_or_slide_res {
                            FallOrSlide::Fall => {
                                entity.base_entity.physics.falling = true;
                            },
                            FallOrSlide::Slide(z_dist) => {
                                entity.base_entity.physics.pos.z -= z_dist;
                            }
                        };
                    }
                }
            }
        };
    }
}

enum FallOrSlide {
    Fall,
    Slide(f32)
}

fn fall_or_slide(point: &v3, world: &[Triangle], res: &mut FallOrSlide) {
    let mut set_falling = true;
    // If not falling and close to ground, snap to ground, to avoid jitter

    let mut max_z = 0.0;
    for triangle in world {
        let projection  = triangle.project_point_z_axis(&point);

        let inside = triangle.inside(&projection);
        let z_diff = point.z - projection.z;

        // close to gorund maybe don't fall
        // play with this fall heigt
        if inside {
            //println!("InSide z_diff {:?}", z_diff);
            if z_diff < 0.3 {

                set_falling = false;
                // to avoid jitter don't snap when too close to ground
                if z_diff > 0.01 {
                    max_z = f32::max(max_z, z_diff);
                }
            }
        }
    }


    if !set_falling {
        let mut new_dist = max_z;
        match res {
            FallOrSlide::Slide(z_dist) => {
                new_dist = f32::min(*z_dist, max_z);
            },
            _ => {}
        }
        *res = FallOrSlide::Slide(new_dist);
    }

}
