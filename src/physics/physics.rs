use crate::game;
use crate::physics::impulse_resolution::*;


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
