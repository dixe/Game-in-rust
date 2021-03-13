use nalgebra as na;

use crate::entity;
use crate::game;


use crate::physics::projection_collision::*;


#[derive(Copy, Clone, Debug)]
pub struct Manifold {
    entity_1_index: usize,
    entity_2_index: usize ,
    penetration: f32,
    normal: na::Vector3::<f32>
}


pub fn do_impulse_correction(ctx: &mut game::Context) {
    let (mut impulse_entities, collision_shapes) = create_impulse_entities(ctx);

    let manifolds = do_impulse_collisions(&mut impulse_entities, &collision_shapes);

    impulse_collisions_resolution(ctx, manifolds, &mut impulse_entities);

    for e in impulse_entities {
        if e.entity_id > 0 { // don't update walls ect
            ctx.ecs.set_physics(e.entity_id, e);
        }
    }

}


fn impulse_collisions_resolution(ctx: &mut game::Context, manifolds: Vec<Manifold>,  entities: &mut Vec<entity::Physics>) {
    for manifold in manifolds {
        let mut e1 = entities[manifold.entity_1_index];
        let mut e2 = entities[manifold.entity_2_index];
        //println!("E1 BEFORE {:#?}", e2);
        impulse_collision_resolution(&mut e1, &mut e2, &manifold);
        impulse_position_correction(&mut e1, &mut e2, &manifold);
        //println!("E1 AFTER {:#?}", e2);
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


    /*
    if vel_along_normal != 0.0 {
    println!("\n\n\nNEW ONE STARTS HERE");
    println!("NORMAL: {:#?}", manifold.normal);
    println!("RELATIVE VEL: {:#?}", relative_vel);
    println!("VEL_ALONG_NORMAL: {}", vel_along_normal);

    println!("E1 VEL: {}", entity_1.velocity);
    println!("E2 VEL: {}", entity_2.velocity);

}

     */

    // resittution, how much energy is transferend back, should be between 0 and 1
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




fn create_impulse_entities(ctx: &game::Context) -> (Vec<entity::Physics>, Vec<ConvexCollisionShape>) {

    let mut entities = Vec::new();
    let mut collision_shapes = Vec::new();
    // Get some data out of enitiy component system
    match ctx.ecs.get_physics(ctx.player_id) {
        Some(p) => {
            entities.push(*p);
            collision_shapes.push(ConvexCollisionShape::rectangle(&p.pos, 1.0, 1.0, p.rotation_cos, p.rotation_sin));
            //println!("sing, cos {}, {}", p.rotation_cos, p.rotation_sin);
        },
        None => {},
    };


    for enemy_id in &ctx.enemies {
        match ctx.ecs.get_physics(*enemy_id) {
            Some(en) => {
                entities.push(*en);
                collision_shapes.push(ConvexCollisionShape::rectangle(&en.pos, 1.0, 1.0, en.rotation_cos, en.rotation_sin));
            },

            None => continue
        };
    }

    for w in ctx.scene.border_sides() {
        // todo have these stored in the scene along side the collision shapes
        let mut entity = entity::Physics::new(usize::MIN, usize::MIN);

        entity.inverse_mass = 0.0;

        entities.push(entity);
        collision_shapes.push(w.clone());

    }


    //TODO also take the scene into account with inverse mass 0. This is the same as infinite mass

    // TODO Maybe also add projeciles, if htey should push a little bit

    (entities, collision_shapes)

}

fn do_impulse_collisions(entities: &[entity::Physics], shapes: &[ConvexCollisionShape]) -> Vec<Manifold> {

    let mut res = Vec::new();

    for index_1 in 0..entities.len() {
        let e1 = entities[index_1];
        for index_2 in (index_1+1)..entities.len() {
            let e2 = entities[index_2];
            // if both are a wall we don't care about collision
            if e1.entity_id == 0 && e2.entity_id == 0 {
                continue;
            }
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
