use crate::game;
use crate::physics;
use crate::entity;
use crate::action_system;
use crate::controls;
use crate::camera;


fn format_matrix4(mat: &na::Matrix4::<f32>) {

    println!("{:.2} {:.2} {:.2} {:.2} ", mat[0], mat[1], mat[2], mat[3]);
    println!("{:.2} {:.2} {:.2} {:.2} ", mat[4], mat[5], mat[6], mat[7]);
    println!("{:.2} {:.2} {:.2} {:.2} ", mat[8], mat[9], mat[10], mat[11]);
    println!("{:.2} {:.2} {:.2} {:.2} ", mat[12], mat[13], mat[14], mat[15]);
}

fn format_matrix3(mat: &na::Matrix3::<f32>) {

    println!("{:.2} {:.2} {:.2} ", mat[0], mat[1], mat[2]);
    println!("{:.2} {:.2} {:.2} ", mat[3], mat[4], mat[5]);
    println!("{:.2} {:.2} {:.2} ", mat[6], mat[7], mat[8]);
}

pub fn update_game_state(ctx: &mut game::Context, collisions: &Vec<physics::EntityCollision>) {


    let delta = ctx.get_delta_time();


    // also "action" system update fx sword arc ect
    //action_system::update_actions(&mut ctx.ecs.actions_info, &mut ctx.ecs.physics, &mut ctx.state, delta as f32, &ctx.actions);


    // PLAYER MOVEMENT

    update_player(ctx.cameras.current(), &ctx.controls, ctx.entities.player_mut(), delta);


    // make a function on player, weapon anchor mat and just use that as world_matrix
    let mut world_mat = ctx.entities.player().skeleton.joints[14].world_matrix;
    // This is not it

    let player_model_mat = ctx.entities.player().physics.calculate_model_mat();
    let hammer = ctx.entities.hammer_mut();

    hammer.physics.apply_transform(player_model_mat * world_mat);

    /*
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
     */


}


fn update_player(camera: &dyn camera::Camera, controls: &controls::Controls, player: &mut entity::Entity,  _delta:  f32) {

    // UPDATE STATE, IE WHEN ATTACK IS DONE SET BACK TO IDLE
    update_player_state(player);


    if !can_perform_action(player.get_state()) {
        game::update_velocity(&mut player.physics, na::Vector3::new(0.0, 0.0, 0.0));
        return;
    }

    if controls.attack {
        player.update_state(entity::EntityState::Attack);
        return;
    }

    match camera.mode() {
        camera::CameraMode::Follow => {
            let z_rot = camera.z_rotation();

            let rot_mat = na::Matrix3::new_rotation(z_rot);
            let player_move_dir = rot_mat * na::Vector3::new(-controls.movement_dir.y, controls.movement_dir.x, 0.0);

            game::update_velocity(&mut player.physics, player_move_dir);

            if player_move_dir.magnitude() > 0.0 {
                player.physics.target_dir = player_move_dir.normalize();
            }

            let mut target_state = entity::EntityState::Idle;


            if player.physics.velocity.magnitude() > 0.0 {

                target_state = entity::EntityState::Moving;
            }



            if player.get_state() != target_state {

                player.update_state(target_state);
            }



        },

        camera::CameraMode::Free => {},
    }
}

fn can_perform_action(state: entity::EntityState) -> bool {
    match state {
        entity::EntityState::Idle => true,
        entity::EntityState::Moving => true,
        entity::EntityState::Attack => false,
    }
}


fn update_player_state(player: &mut entity::Entity) {
    match player.get_state() {
        entity::EntityState::Attack => {

            if player.animation_player.as_ref().unwrap().has_repeated {
                player.update_state(entity::EntityState::Idle);
            }

            // check if animation is don
        },
        _ => {}
    };
}



fn update_enemies_death(ctx: &mut game::Context) {
    /*
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
     */
}
