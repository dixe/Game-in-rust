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
    let player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(p) => *p,
        None => return None,
    };


    let mut enemies = std::collections::HashMap::with_capacity(ctx.enemies.len());

    for enemy_id in &ctx.enemies {
        match ctx.ecs.get_physics(*enemy_id) {
            Some(en) => enemies.insert(*enemy_id, *en),
            None => continue
        };
    }


    let mut projectiles = std::collections::HashMap::with_capacity(ctx.player_projectiles.len());

    for projectile in &ctx.player_projectiles {
        match ctx.ecs.get_physics(projectile.entity_id) {
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



    let mut col_entities = match create_collision_entities(ctx) {
        Some(col_en) => col_en,
        None => return Collisions {
            enemies_hit: Vec::<Hit>::new(),
            player_enemy_collision: std::collections::HashMap::new(),
        }
    };

    let delta = ctx.get_delta_time();




    // UPDATE POSITION AND DETECT COLLISIONS
    update_entities_position(&mut col_entities, delta);

    let collisions = do_collisions(&mut col_entities, &ctx.scene);


    // HANDLE ENTITY COLLISIONS
    handle_collisions(&collisions, &mut col_entities, &ctx.scene);



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

fn do_collisions(entities: &mut CollisionEntities, scene: &scene::Scene) -> Collisions {

    let mut collisions = Collisions {
        enemies_hit: Vec::<Hit>::new(),
        player_enemy_collision: std::collections::HashMap::new(),
    };



    entity_update_movement_scene(&mut entities.player, scene);

    // updated to be inside the walls

    let player_collision = ConvexCollisionShape::rectangle(&mut entities.player.pos, 1.0, 1.0);

    for enemy in entities.enemies.values_mut() {
        entity_update_movement_scene(enemy, scene);


        let enemy_collision = ConvexCollisionShape::rectangle(&enemy.pos, 1.0, 1.0);

        let (col, dir) = collision_sat_shapes(&player_collision, &enemy_collision);
        if col {
            collisions.player_enemy_collision.insert(enemy.entity_id, dir);
        }

        for projectile in entities.projectiles.values() {

            let projectile_collision = ConvexCollisionShape::rectangle(&projectile.pos, 1.0, 1.0);

            let (col, _) = collision_sat_shapes(&projectile_collision, &enemy_collision);

            if col {
                collisions.enemies_hit.push(Hit { entity_id: enemy.entity_id, projectile_id: projectile.entity_id});
            }
        }

    }

    collisions

}


fn handle_collisions(collisions: &Collisions, entities: &mut CollisionEntities, scene: &scene::Scene) {

    for (enemy_id, dir) in &collisions.player_enemy_collision {

        let enemy = match entities.enemies.get_mut(enemy_id) {
            Some (e) => e ,
            None => continue,
        };


        // do the push on enenmy
        enemy.pos += dir;

        // check if there is a collision after we did this



        let (col, correction) = entity_update_movement_scene(enemy, scene);
        if col {
            entities.player.pos -= correction;
        };

    }
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
