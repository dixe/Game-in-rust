use nalgebra as na;

use crate::entity;
use crate::game;


use crate::physics::projection_collision::*;

use crate::physics::physics::EntityCollision;


#[derive(Copy, Clone, Debug)]
pub struct Manifold {
    entity_1_index: usize,
    entity_2_index: usize ,
    penetration: f32,
    normal: na::Vector3::<f32>
}



pub fn do_impulse_correction(ctx: &mut game::Context) -> Vec<EntityCollision>{

    let (mut impulse_entities, collision_shapes, no_checks) = create_impulse_entities(ctx);

    let manifolds = do_impulse_collisions(&mut impulse_entities, &collision_shapes, no_checks);

    impulse_collisions_resolution(&manifolds, &mut impulse_entities);

    for e in impulse_entities.iter() {

        // Find a better way to not update walls
        // and update entities in general

        //ctx.entities.set_physics(e.entity_id, *e);

    }

    manifolds.iter().filter_map(|m| {
        let e1 = impulse_entities[m.entity_1_index];
        let e2 = impulse_entities[m.entity_2_index];


        Some(EntityCollision {
            entity_1_id: 0,
            entity_2_id: 0,
        })
    }).collect()


}


fn impulse_collisions_resolution( manifolds: &Vec<Manifold>, entities: &mut Vec<entity::Physics>) {
    for manifold in manifolds {
        let mut e1 = entities[manifold.entity_1_index];
        let mut e2 = entities[manifold.entity_2_index];

        impulse_collision_resolution(&mut e1, &mut e2, &manifold);
        impulse_position_correction(&mut e1, &mut e2, &manifold);

        let _ = std::mem::replace(&mut entities[manifold.entity_1_index], e1);
        let _ = std::mem::replace(&mut entities[manifold.entity_2_index], e2);
    }
}

fn impulse_collision_resolution(entity_1: &mut entity::Physics, entity_2: &mut entity::Physics, manifold: &Manifold) {
    // from https://gamedevelopment.tutsplus.com/tutorials/how-to-create-a-custom-2d-physics-engine-the-basics-and-impulse-resolution--gamedev-6331


    let relative_vel = entity_2.velocity - entity_1.velocity;

    let vel_along_normal = relative_vel.dot(&manifold.normal);
    if vel_along_normal > 0.0 {

        // println!("{:#?}", vel_along_normal);
        return ;
    }


    // restitution, how much energy is transferend back, should be between 0 and 1
    // 1 is no energy absorbed and everything goes to new velocity
    // with 1 object in motion does not lose energy when hitting fx a wall
    //
    let e = 0.6;

    // impulse scalar
    let mut j =  -(1.0 + e) * vel_along_normal;
    j /= entity_1.inverse_mass + entity_2.inverse_mass;


    let impulse = j * manifold.normal;
    entity_1.velocity -= entity_1.inverse_mass * impulse;
    entity_2.velocity += entity_2.inverse_mass * impulse;
}


fn impulse_position_correction(entity_1: &mut entity::Physics, entity_2: &mut entity::Physics, manifold: &Manifold) {

    let percent = 0.8;
    let slop = 0.01;
    let correction = (f32::max(manifold.penetration - slop, 0.0) / (entity_1.inverse_mass + entity_2.inverse_mass)) * percent * manifold.normal;

    entity_1.pos -= entity_1.inverse_mass * correction;
    entity_2.pos += entity_2.inverse_mass * correction;
}



fn create_impulse_entities(ctx: &game::Context) -> (Vec<entity::Physics>, Vec<ConvexCollisionShape>, std::collections::HashSet<(usize,usize)>) {

    let mut entities = Vec::new();
    let mut collision_shapes = Vec::new();
    let mut no_checks = std::collections::HashSet::new();

    // Get some data out of enitiy component system
    let player_physics = ctx.entities.player.physics;
    entities.push(player_physics);
    collision_shapes.push(ConvexCollisionShape::rectangle(&player_physics.pos, 1.0, 1.0, &player_physics));



    /*
    for enemy_id in &ctx.state.enemies {
    match ctx.ecs.get_physics(*enemy_id) {
    Some(en) => {
    entities.push(*en);
    collision_shapes.push(ConvexCollisionShape::rectangle(&en.pos, 1.0, 1.0, en ));
},
    None => continue
};
}

    // ADD PLAYER PROJECTILES AS IMPULSE ENTITIES
    for shot in &ctx.state.player_shots {
    match ctx.ecs.get_physics(*shot) {
    Some(proj) => {

    no_checks.insert((ctx.player_id, proj.entity_id));

    entities.push(*proj);
    collision_shapes.push(ConvexCollisionShape::rectangle(&proj.pos, 1.0, 1.0, proj));
},
    None => continue
}
}

    // ADD PLAYER PROJECTILES AS IMPULSE ENTITIES
    for shot in &ctx.state.enemy_shots {
    match ctx.ecs.get_physics(*shot) {
    Some(proj) => {
    for e in &ctx.state.enemies {
    no_checks.insert((*e, proj.entity_id));

}
    entities.push(*proj);
    collision_shapes.push(ConvexCollisionShape::rectangle(&proj.pos, 1.0, 1.0, proj));
},
    None => continue
}
}

    for w in ctx.scene.border_sides() {
    // todo have these stored in the scene along side the collision shapes
    let mut entity = entity::Physics::new(usize::MIN);

    entity.inverse_mass = 0.0;

    entities.push(entity);
    collision_shapes.push(w.clone());
}
     */



    (entities, collision_shapes, no_checks)

}



fn do_impulse_collisions(entities: &[entity::Physics], shapes: &[ConvexCollisionShape], no_checks: std::collections::HashSet<(usize,usize)> ) -> Vec<Manifold> {

    let mut res = Vec::new();

    for index_1 in 0..entities.len() {
        let e1 = entities[index_1];
        for index_2 in (index_1+1)..entities.len() {
            let e2 = entities[index_2];
            // if both are a wall we don't care about collision
            // TODO again find a better way, maybe just have a bool that says wall ??
            /*
            if e1.entity_id == 0 && e2.entity_id == 0 {
            continue;
        }
             */


            let (col, normal, penetration) = collision_sat_shapes_impulse(&shapes[index_1], &shapes[index_2]);


            if col {
                // println!("{} - {}", index_1, index_2);
                res.push(Manifold {
                    entity_1_index: index_1,
                    entity_2_index: index_2,
                    normal,
                    penetration
                });
            }
        }
    }

    //println!("{:#?}", res);
    res

}
