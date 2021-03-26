use crate::game;
use crate::physics;
use crate::entity;
use crate::action_system;
use crate::controls;

pub fn update_game_state(ctx: &mut game::Context, collisions: &Vec<physics::EntityCollision>) {

    let delta = ctx.get_delta_time();

    // Update shooters, projectiles and othertime based stuff
    update_projectiles(ctx, delta);
    update_shooters(ctx, delta);
    update_enemies_death(ctx);
    //update_player_shooting(ctx);
    update_player_swing(ctx);

    // also "action" system update fx sword arc ect
    action_system::update_actions(&mut ctx.ecs.actions_info, &mut ctx.ecs.physics, &mut ctx.state, delta as f32, &ctx.actions);


    // PLAYER MOVEMENT

    let mut player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(e) => *e,
        None => return, // Dead player, don't care about ai update
    };

    update_player_movement(ctx, &mut player);




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


fn update_player_movement(ctx: &mut game::Context, player: &mut entity::Physics) {
    if ctx.state.player_state != game::PlayerState::Moving {
        game::update_velocity(player, na::Vector3::new(0.0, 0.0, 0.0));
        return;
    }

    match ctx.controls.cam_mode {
        controls::CameraMode::TopDown => {
            game::update_velocity(player, ctx.controls.movement_dir);
        },
        controls::CameraMode::Follow => {

            let z_rot = ctx.camera.z_rotation();

            let rot_mat = na::Matrix3::new_rotation(z_rot);
            let player_move_dir = rot_mat * na::Vector3::new(-ctx.controls.movement_dir.y, ctx.controls.movement_dir.x, 0.0);

            game::update_velocity(player, player_move_dir);
        }
    }


    if player.velocity.magnitude() > 0.0 {
        game::update_rotation(player, player.velocity);
    }

}

fn update_enemies_death(ctx: &mut game::Context) {
    let mut deaths = Vec::new();
    for e in &ctx.state.enemies {
        let enemy_hp = match ctx.ecs.get_health(*e) {
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


fn update_player_swing(ctx: &mut game::Context) {

    if ctx.state.player_state != game::PlayerState::Moving {
        return;
    }

    if ! ctx.controls.right_shoulder {
        return;
    }

    // start swing action/animaiton and set somekind of state to swinging, so we cannot start new action

    let init_physics = match ctx.ecs.get_physics(ctx.player_weapon_id) {
        Some(p) => *p,
        _=> return
    };

    let mut action_info =
    //match ctx.ecs.get_actions_info(ctx.player_weapon_id) {
        match ctx.ecs.actions_info.get_mut(&ctx.player_weapon_id) {
            Some(info) => info,
            _ => return
        };


    let mut swing_action = entity::ActionData::new(action_system::Actions::Swing, Some(action_system::set_player_moving), init_physics);
    swing_action.total_time = 0.6;

    // TODO look at the current state and maybe add to queue instead of as current

    action_info.queue.push_back(swing_action);
    ctx.state.player_state = game::PlayerState::Attacking;


}

fn update_player_shooting(ctx: &mut game::Context) {

    let player_id = ctx.player_id;
    let shoot_dir = ctx.controls.right_stick;
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
