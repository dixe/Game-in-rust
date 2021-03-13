use nalgebra as na;

use crate::game;
use crate::physics::impulse_resolution::*;


pub struct Collisions {
    pub enemies_hit: Vec<Hit>,
    pub player_enemy_collision: std::collections::HashMap<usize, na::Vector3::<f32>>,
}

pub struct Hit {

    pub entity_id: usize,
    pub projectile_id: usize,
}





pub fn process(ctx: &mut game::Context) -> Collisions {
    // MOVE ENTITIES
    update_entities_position(ctx);


    //DO IMPULSE COLLISION AND UPDATE
    do_impulse_correction(ctx);

    let collisions = Collisions {
        enemies_hit: Vec::<Hit>::new(),
        player_enemy_collision: std::collections::HashMap::new(),

    };

    collisions

}


fn update_entities_position(ctx: &mut game::Context) {
    let delta = ctx.get_delta_time();

    for physics in ctx.ecs.physics.values_mut() {
        physics.pos += physics.velocity * delta;
    }

}
