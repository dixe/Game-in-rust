use crate::game;
use crate::entity;
use crate::physics::impulse_resolution::*;
use crate::physics::projection_collision::*;


#[derive(Copy, Clone, Debug)]
pub struct EntityCollision {
    pub entity_1_id: usize,
    pub entity_2_id: usize
}


pub fn process(ctx: &mut game::Context) -> Vec<EntityCollision> {
    // MOVE ENTITIES
    update_entities_position(ctx);


    //DO IMPULSE COLLISION AND UPDATE
    let impulse_collisions = do_impulse_correction(ctx);


    impulse_collisions

}


fn update_entities_position(ctx: &mut game::Context) {
    let delta = ctx.get_delta_time();

    for physics in ctx.ecs.physics.values_mut() {
        physics.pos += physics.velocity * delta;
    }
}



/*

fn add_collision_shapes(ctx: &game::Context, id: usize, data: &mut Vec<(entity::Physics, ConvexCollisionShape)>) {

match ctx.ecs.get_entity_type(id) {
None => add_collision_shape_simple(ctx, id, data),
Some(entity::EntityType::Simple(_)) => add_collision_shape_simple(ctx, id, data),
Some(entity::EntityType::Complex(complex)) => {

for sub_id in &complex.sub_entities {

// calculate the correct physcis pos based on base model

let anchor_physics =  match ctx.ecs.get_physics(id) {
Some(entity) => *entity,
None => {return}
                };

                add_collision_shape_simple(ctx, id, data);
                add_collision_shapes(ctx, *sub_id, data);

            }
        }
    };

}


fn add_collision_shape_simple(ctx: &game::Context, id: usize, data: &mut Vec<(entity::Physics, ConvexCollisionShape)>) {
    match ctx.ecs.get_physics(id) {
        Some(entity) => data.push((*entity, ConvexCollisionShape::rectangle(&entity.pos, 1.0, 1.0, entity))),
        None => {}
    };
}

fn add_collision_shapes_simple(ctx: &game::Context, id: usize, entities: &mut Vec<entity::Physics>, collision_shapes: &mut Vec<ConvexCollisionShape>, no_checks: &mut std::collections::HashSet<(usize,usize)>) {

    match ctx.ecs.get_physics(id) {
        Some(entity) => {
            entities.push(*entity);
            collision_shapes.push(ConvexCollisionShape::rectangle(&entity.pos, 1.0, 1.0, entity ));
        },
        None => {},
    };

}
*/
