use nalgebra as na;

use crate::game;
use crate::entity;


pub fn run_ai(ctx: &mut game::Context) {

    let mut player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(e) => *e,
        None => return, // Dead player, don't care about ai update
    };


    for e in &ctx.enemies {
        let mut enemy = match ctx.ecs.get_physics(*e) {
            Some(e) => *e,
            None => continue, // Dead player, don't care about ai update
        };

        let move_dir = (player.pos - enemy.pos).normalize();
        game::update_velocity_and_rotation(&mut enemy, move_dir);
        ctx.ecs.set_physics(*e, enemy);

    }
}
