
use crate::scene;
use crate::entity;
use crate::game;

use crate::physics::projection_collision::*;


pub struct Collisions {
    pub enemies_hit: Vec<Hit>,
    pub player_enemies_collision: Vec<usize>,
}





pub struct Hit {

    pub entity_id: usize,
    pub projectile_id: usize,

}


pub fn process(ctx: &mut game::Context) -> Collisions {

    let mut collisions = Collisions {
        enemies_hit: Vec::<Hit>::new(),
        player_enemies_collision: Vec::<usize>::new(),
    };

    let delta = ctx.get_delta_time();

    let mut player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(p) => *p,
        None => return collisions
    };


    // todo take collision_shape and return collision. i.e. dont make it mutable
    entity_update_movement_scene(&mut player, delta, &ctx.scene);


    let player_collision = ConvexCollisionShape::generate_rectangle_collision_shape(&player.pos, 1.0, 1.0);

    for enemy_id in &mut ctx.enemies {
        let mut enemy = match ctx.ecs.get_physics(*enemy_id) {
            Some(en) => *en,
            None => continue
        };

        let enemy_collision = ConvexCollisionShape::generate_rectangle_collision_shape(&enemy.pos, 1.0, 1.0);

        entity_update_movement_scene(&mut enemy, delta, &ctx.scene);

        for proj in &mut ctx.player_projectiles {
            let mut proj_physics = match ctx.ecs.get_physics(proj.entity_id) {
                Some(e) => *e,
                None => continue
            };

            let proj_collision = ConvexCollisionShape::generate_rectangle_collision_shape(&proj_physics.pos, 1.0, 1.0);


            proj_physics.set_position(proj_physics.pos + proj_physics.velocity*delta);
            ctx.ecs.set_physics(proj.entity_id, proj_physics);

            let (col, _) = collision_sat_shapes(&proj_collision, &enemy_collision);
            if col {
                collisions.enemies_hit.push(Hit { entity_id: *enemy_id, projectile_id: proj.entity_id});
            }

        }

        let (col, dir) = collision_sat_shapes(&player_collision, &enemy_collision);

        if col {
            enemy.pos -= dir;
            collisions.player_enemies_collision.push(*enemy_id);
        }

        ctx.ecs.set_physics(*enemy_id, enemy);
    }


    for proj in &mut ctx.player_projectiles {
        let mut projectile = match ctx.ecs.get_physics(proj.entity_id) {
            Some(e) => *e,
            None => continue
        };

        projectile.set_position(projectile.pos + projectile.velocity * delta);
        ctx.ecs.set_physics(proj.entity_id, projectile);
    }

    ctx.ecs.set_physics(ctx.player_id, player);

    collisions
}



fn entity_update_movement_scene(entity: &mut entity::Physics, delta: f32, scene: &scene::Scene) {

    let mut entity_pos_updated = entity.pos + entity.velocity * delta;

    for wall in scene.border_sides() {

        let col_shape = ConvexCollisionShape::generate_rectangle_collision_shape(&entity_pos_updated, 1.0, 1.0);

        let (col, dir) = collision_sat_shapes(&col_shape, &wall);

        if col {
            //println!("{:#?}", dir);
            entity_pos_updated -= dir;
        }
    }

    entity.set_position(entity_pos_updated);
}
