use nalgebra as na;


use crate::scene;
use crate::entity;
use crate::game;

use crate::physics::projection_collision::*;


pub struct Collisions {
    pub enemies_hit: Vec<Hit>,
    pub player_enemy_collision: std::collections::HashMap<usize, na::Vector3::<f32>>,
}



pub struct Hit {

    pub entity_id: usize,
    pub projectile_id: usize,
}


struct CollisionEntities {
    player: entity::Physics,
    projectiles: std::collections::HashMap::<usize, entity::Physics>,
    enemies: std::collections::HashMap::<usize, entity::Physics>,
}


fn create_collision_entities(ctx: &game::Context) -> Option<CollisionEntities> {
    // Get some data out of enitiy component system
    let mut player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(p) => *p,
        None => return None,
    };


    let mut enemies = std::collections::HashMap::with_capacity(ctx.enemies.len());

    for enemy_id in &ctx.enemies {
        let mut enemy = match ctx.ecs.get_physics(*enemy_id) {
            Some(en) => enemies.insert(*enemy_id, *en),
            None => continue
        };
    }


    let mut projectiles = std::collections::HashMap::with_capacity(ctx.player_projectiles.len());

    for projectile in &ctx.player_projectiles {
        let mut projectile = match ctx.ecs.get_physics(projectile.entity_id) {
            Some(proj) => projectiles.insert(projectile.entity_id, *proj),
            None => continue
        };
    }

    Some(CollisionEntities {
        player,
        enemies,
        projectiles
    })

}


pub fn process(ctx: &mut game::Context) -> Collisions {

    let mut collisions = Collisions {
        enemies_hit: Vec::<Hit>::new(),
        player_enemy_collision: std::collections::HashMap::new(),
    };


    let mut col_entities = match create_collision_entities(ctx) {
        Some(col_en) => col_en,
        None => return collisions
    };

    let delta = ctx.get_delta_time();




    // UPDATE POSITION AND DETECT COLLISIONS
    update_entities_position(&mut col_entities, delta);

    //    let collisions = do_collisions(&mut player, &mut enemies,  &mut projectiles, &ctx.scene);


    // HANDLE ENTITY COLLISIONS
    //    handle_collisions(&collisions, &mut player, &mut enemies, &ctx.scene);



    // SET THE CALCULATED PHYSICS
    for enemy in col_entities.enemies.values_mut() {
        ctx.ecs.set_physics(enemy.entity_id, *enemy);

    }

    for projectile in col_entities.projectiles.values_mut() {
        ctx.ecs.set_physics(projectile.entity_id, *projectile);
    }

    ctx.ecs.set_physics(ctx.player_id, col_entities.player);
    collisions

}


fn update_entities_position(entities: &mut CollisionEntities, delta: f32) {

    update_entity_position(&mut entities.player, delta);

    for entity in entities.enemies.values_mut() {
        update_entity_position(entity, delta);
    }


    for entity in entities.projectiles.values_mut() {
        update_entity_position(entity, delta);
    }
}

fn update_entity_position(entity: &mut entity::Physics, delta: f32) {
    entity.pos += entity.velocity * delta;
}

fn do_collisions(player: &mut entity::Physics,
                 enemies: &mut std::collections::HashMap::<usize, entity::Physics>,
                 projectiles: &mut std::collections::HashMap::<usize, entity::Physics>,
                 scene: &scene::Scene) -> Collisions {

    let mut collisions = Collisions {
        enemies_hit: Vec::<Hit>::new(),
        player_enemy_collision: std::collections::HashMap::new(),
    };

    entity_update_movement_scene(player, scene);

    // updated to be inside the walls

    let player_collision = ConvexCollisionShape::rectangle(&player.pos, 1.0, 1.0);

    for enemy in enemies.values_mut() {
        entity_update_movement_scene(enemy, scene);


        let enemy_collision = ConvexCollisionShape::rectangle(&enemy.pos, 1.0, 1.0);

        let (col, dir) = collision_sat_shapes(&player_collision, &enemy_collision);
        if col {
            collisions.player_enemy_collision.insert(enemy.entity_id, dir);
        }

        for projectile in projectiles.values() {

            let projectile_collision = ConvexCollisionShape::rectangle(&projectile.pos, 1.0, 1.0);

            let (col, _) = collision_sat_shapes(&projectile_collision, &enemy_collision);

            if col {
                collisions.enemies_hit.push(Hit { entity_id: enemy.entity_id, projectile_id: projectile.entity_id});
            }
        }

    }

    collisions

}


fn handle_collisions(collisions: &Collisions, player: &mut entity::Physics,enemies: &mut std::collections::HashMap::<usize, entity::Physics>, scene: &scene::Scene) {

    for (enemy_id, dir) in &collisions.player_enemy_collision {

        let enemy = match enemies.get_mut(enemy_id) {
            Some (e) => e ,
            None => continue,
        };


        // do the push on enenmy
        enemy.pos += dir;

        // check if there is a collision after we did this



        let (col, correction) = entity_update_movement_scene(enemy, scene);
        if col {
            player.pos -= correction;
        };

    }
}



fn do_collisions_old(ctx: &mut game::Context) -> Collisions {

    let mut collisions = Collisions {
        enemies_hit: Vec::<Hit>::new(),
        player_enemy_collision: std::collections::HashMap::new(),
    };

    let delta = ctx.get_delta_time();

    let mut player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(p) => *p,
        None => return collisions
    };


    entity_update_movement_scene(&mut player, &ctx.scene);

    // has to be after because update_movement alters player position. But only for the durration of physics calc
    let player_collision = ConvexCollisionShape::rectangle(&player.pos, 1.0, 1.0);

    for enemy_id in &mut ctx.enemies {
        let mut enemy = match ctx.ecs.get_physics(*enemy_id) {
            Some(en) => *en,
            None => continue
        };

        entity_update_movement_scene(&mut enemy, &ctx.scene);

        let enemy_collision = ConvexCollisionShape::rectangle(&enemy.pos, 1.0, 1.0);

        for proj in &mut ctx.player_projectiles {
            let mut proj_physics = match ctx.ecs.get_physics(proj.entity_id) {
                Some(e) => *e,
                None => continue
            };

            let proj_collision = ConvexCollisionShape::rectangle(&proj_physics.pos, 1.0, 1.0);


            proj_physics.set_position(proj_physics.pos + proj_physics.velocity*delta);
            ctx.ecs.set_physics(proj.entity_id, proj_physics);

            let (col, _) = collision_sat_shapes(&proj_collision, &enemy_collision);
            if col {
                collisions.enemies_hit.push(Hit { entity_id: *enemy_id, projectile_id: proj.entity_id});
            }

        }

        let (col, dir) = collision_sat_shapes(&player_collision, &enemy_collision);

        if col {
            enemy.pos += dir;
            collisions.player_enemy_collision.insert(*enemy_id, dir);
        }

        ctx.ecs.set_physics(*enemy_id, enemy);
    }


    for proj in &mut ctx.player_projectiles {
        let mut projectile = match ctx.ecs.get_physics(proj.entity_id) {
            Some(e) => *e,
            None => continue
        };

        projectile.set_position(projectile.pos + projectile.velocity * delta);
    }

    collisions
}





fn entity_update_movement_scene( entity: &mut entity::Physics, scene: &scene::Scene) -> (bool, na::Vector3::<f32>) {

    let mut entity_pos_updated = entity.pos;
    let mut hit = false;
    let mut wall_correction = na::Vector3::new(0.0, 0.0, 0.0);

    for wall in scene.border_sides() {

        let col_shape = ConvexCollisionShape::rectangle(&entity_pos_updated, 1.0, 1.0);

        let (col, correction) = collision_sat_shapes(&col_shape, &wall);

        if col {
            //println!("{:#?}", dir);
            entity_pos_updated -= correction;
            wall_correction += correction;
            hit = true;
        }
    }

    //TODO should we do this?? I makes sence that we don't use a illegal position for the rest of the update code
    entity.set_position(entity_pos_updated);
    (hit, wall_correction)
}
