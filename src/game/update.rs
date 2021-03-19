use crate::game;
use crate::physics;
use crate::entity;
use crate::animation_system;


pub fn update_game_state(ctx: &mut game::Context, collisions: &Vec<physics::EntityCollision>) {

    let delta = ctx.get_delta_time();


    // Update shooters, projectiles and othertime based stuff
    update_projectiles(ctx, delta);
    update_shooters(ctx, delta);
    update_enemies_death(ctx);
    update_player_shooting(ctx);

    // also "animation" system update fx sword arc ect
    animation_system::update_animations(&mut ctx.ecs.animations_info, &mut ctx.ecs.physics, delta as f32);


    // PLAYER MOVEMENT

    let mut player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(e) => *e,
        None => return, // Dead player, don't care about ai update
    };


    game::update_velocity_and_rotation(&mut player, ctx.controls.movement_dir);

    ctx.ecs.set_physics(ctx.player_id, player);

    for c in collisions {
        // VALIDATE! entity_1 is always lower than entity_2 and enemies will spawn before projectiles always??

        if ctx.state.enemies.contains(&c.entity_1_id) && ctx.state.player_shots.contains(&c.entity_2_id) {
            match (ctx.ecs.get_health(c.entity_1_id), ctx.ecs.get_shot(c.entity_2_id )) {
                (Some(e_hp), Some(s)) => {
                    let mut shot = *s;
                    let mut enemy_hp = *e_hp;

                    if ! shot.used {
                        enemy_hp.damage(shot.damage);
                        shot.used = true;
                        ctx.ecs.set_shot(shot.entity_id, shot);
                        ctx.ecs.set_health(c.entity_1_id, enemy_hp);

                    }

                },
                _ => {}

            };

        };
    }
}



fn update_enemies_death(ctx: &mut game::Context) {
    let mut deaths = Vec::new();
    for e in &ctx.state.enemies {
        let mut enemy_hp = match ctx.ecs.get_health(*e) {
            Some(e_hp) => *e_hp,
            None => continue,
        };

        if enemy_hp.health() <= 0.0 {
            deaths.push(e);
        }
    }

    for dead in deaths {
        ctx.ecs.remove_entity(*dead);
    }
}


fn update_shooters(ctx: &mut game::Context, delta: f32) {

    for shooter in ctx.ecs.shooter.values_mut() {
        shooter.update(delta);
    }
}



fn update_projectiles(ctx: &mut game::Context, delta: f32) {
    let mut remove_shots = Vec::new();
    for shot in &mut ctx.ecs.shot.values_mut() {

        shot.update(delta);

        if shot.expired {
            remove_shots.push(shot.entity_id);
        }
    }


    for remove_shot_id in &remove_shots {
        ctx.state.remove_shot(remove_shot_id);
    }


    // enemies shot when needed
}


fn update_player_shooting(ctx: &mut game::Context) {

    let player_id = ctx.player_id;
    let shoot_dir = ctx.controls.shoot_dir;
    let shooter_op = ctx.ecs.get_shooter(ctx.player_id);
    match (shoot_dir, shooter_op) {
        (Some(dir), Some(s)) =>
        {
            let shooter = *s;
            game::add_projectile(&mut ctx.state.player_shots, &mut ctx.ecs, &shooter, dir, player_id, ctx.projectile_model_id);

        },
        _ => {}
    };

}
