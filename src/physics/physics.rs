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

    //NON IMPULSE COLLISION

    let weapon_col_shape = create_entity_collision_shape(ctx.player_weapon_id, ctx);

    let mut enemies = Vec::<(usize, ConvexCollisionShape)>::new();
    for enemy_id in &ctx.state.enemies {
        match create_entity_collision_shape(*enemy_id, ctx) {
            Some(col_shape) => {
                enemies.push((*enemy_id, col_shape));
            },
            None => continue
        };
    }


    // ONLY DO WHEN PLAYER IS ATTACKING
    // ALSO MAYBE MOVE TO NOT PHYSCIS, SINCE WE DON'T
    // HAVE IT HERE SINCE NO PHYSICS IS GOING ON
    // ALSO WHEN PLAYER ATTACKING IS NOT A PHYSICS CONCERN
    match weapon_col_shape {
        Some(weapon) => {
            weapon_collision(&weapon, enemies);
        },
        _ => {}
    };





    impulse_collisions

}



fn weapon_collision(weapon: &ConvexCollisionShape, enemies: Vec::<(usize, ConvexCollisionShape)>) {

    for (id, enemy) in &enemies {
        let (col, _, _) = collision_sat_shapes_impulse(weapon, &enemy);
        if col {
            println!("Hit yuo stupid");
        }
    }
}


fn update_entities_position(ctx: &mut game::Context) {
    let delta = ctx.get_delta_time();

    for physics in ctx.ecs.physics.values_mut() {
        physics.pos += physics.velocity * delta;
    }
}

fn create_entity_collision_shape(entity_id: usize, ctx: &game::Context) -> Option<ConvexCollisionShape> {
    game::get_absoulte_physics(entity_id, &ctx.ecs).map(|physics| {
        ConvexCollisionShape::rectangle(&physics.pos, 1.0, 1.0, &physics)
    })
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
