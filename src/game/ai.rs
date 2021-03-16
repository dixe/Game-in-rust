use crate::game;
use crate::entity;

use nalgebra_glm as glm;

pub fn run_ai(ctx: &mut game::Context) {

    let player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(e) => *e,
        None => return, // Dead player, don't care about ai update
    };


    for e_id in &ctx.state.enemies {
        let mut enemy = match ctx.ecs.get_physics(*e_id) {
            Some(e) => *e,
            None => continue, // Dead player, don't care about ai update
        };


        if e_id % 2 == 0 || true{
            distance_ai(&mut enemy, player);
            let shooter = match ctx.ecs.get_shooter(enemy.entity_id) {
                Some(s) => Some(*s),
                None => None
            };
            match shooter {
                Some(shooter) =>{
                    shot_ai(&enemy, &shooter, &player, &mut ctx.state.enemy_shots, &mut ctx.ecs, ctx.projectile_model_id);
                },
                _ => {}
            }


        }
        else{
            collision_ai(&mut enemy, player);
        }

        ctx.ecs.set_physics(*e_id, enemy);

    }

}


fn shot_ai(entity: &entity::Physics,
           shooter: &entity::Shooter,
           player_physics: &entity::Physics,
           projectiles: &mut std::collections::HashSet<usize>,
           ecs: &mut entity::EntityComponentSystem,
           shot_model_id: usize) {

    if ! shooter.can_shoot() {
        return;
    }

    let to_player_vec = player_physics.pos - entity.pos;
    let move_dir = (to_player_vec).normalize();

    let dist = to_player_vec.magnitude();

    //TODO check if we can see, take that as ai input
    if dist < 7.0 {
        game::add_projectile(projectiles, ecs, &shooter, to_player_vec, entity.entity_id, shot_model_id);
    }
}



//maybe make this a trait that enemies can have, then have an AI component that implements this trait, or look it up
fn distance_ai(entity: &mut entity::Physics, player_physics: entity::Physics)  {
    let to_player_vec = player_physics.pos - entity.pos;
    let move_dir = (to_player_vec).normalize();

    let target_dist = 5.0;
    let target_point = move_dir * (-target_dist) + player_physics.pos;

    //move_to_point(entity, target_point);
    move_to_point(entity, entity.pos);

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
