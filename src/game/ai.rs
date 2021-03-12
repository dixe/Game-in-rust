use nalgebra as na;

use crate::game;
use crate::entity;


pub fn run_ai(ctx: &mut game::Context) {

    let mut player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(e) => *e,
        None => return, // Dead player, don't care about ai update
    };

    player.velocity = new_velocity(&player, ctx.controls.movement_dir);
    ctx.ecs.set_physics(ctx.player_id, player);



    for e in &ctx.enemies {
        let mut enemy = match ctx.ecs.get_physics(*e) {
            Some(e) => *e,
            None => continue, // Dead player, don't care about ai update
        };

        let move_dir = (player.pos - enemy.pos).normalize();
        //enemy.velocity = new_velocity( &enemy, move_dir);
        ctx.ecs.set_physics(*e, enemy);

    }
}




fn new_velocity( entity: &entity::Physics, new_dir: na::Vector3::<f32>,) -> na::Vector3::<f32> {

    if new_dir.x == 0.0 && new_dir.y == 0.0 && new_dir.z == 0.0 {
        return na::Vector3::new(0.0, 0.0, 0.0);
    }


    let mut new_vel = new_dir.normalize() * entity.acceleration + entity.velocity;

    let speed = new_vel.magnitude();

    if speed > entity.max_speed {
        new_vel *= entity.max_speed / speed;
    }

    new_vel
}
