use crate::game;
use crate::entity;

use nalgebra_glm as glm;

pub fn run_ai(ctx: &mut game::Context) {

    let player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(e) => *e,
        None => return, // Dead player, don't care about ai update
    };


    for e in &ctx.enemies {
        let mut enemy = match ctx.ecs.get_physics(*e) {
            Some(e) => *e,
            None => continue, // Dead player, don't care about ai update
        };


        if e % 2 == 0 {
            distance_ai(&mut enemy, player);
        }
        else{
            collision_ai(&mut enemy, player);
        }

        ctx.ecs.set_physics(*e, enemy);

    }
}




//maybe make this a trait that enemies can have, then have an AI component that implements this trait, or look it up
fn distance_ai(entity: &mut entity::Physics, player_physics: entity::Physics)  {
    let to_player_vec = player_physics.pos - entity.pos;
    let move_dir = (to_player_vec).normalize();

    let target_dist = 3.0;
    let target_point = move_dir * (-target_dist) + player_physics.pos;

    move_to_point(entity, target_point);

}


fn collision_ai(entity: &mut entity::Physics, player_physics: entity::Physics)  {
    move_to_point(entity, player_physics.pos);
}






fn move_to_point(entity: &mut entity::Physics, new_point: na::Vector3<f32>) {

    let move_vec = new_point - entity.pos;

    let move_mag = move_vec.magnitude();

    // when to slow down
    let slow_down_dist = 1.0;
    let smooth = glm::smoothstep(0.0, slow_down_dist, move_mag);
    let target_vel = smooth * move_vec * entity.max_speed;
    let vel_change = target_vel - entity.velocity;

    game::update_velocity_and_rotation(entity, vel_change);

}



#[cfg(test)]
mod tests {

    use crate::game::ai::*;
    use crate::entity;
    use nalgebra as na;


}
