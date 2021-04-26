use crate::game;
use crate::physics;
use crate::entity;
use crate::action_system;
use crate::controls;
use crate::camera;


pub fn update_game_state(ctx: &mut game::Context, collisions: &Vec<physics::EntityCollision>) {


    let delta = ctx.get_delta_time();

    // Update shooters, projectiles and othertime based stuff


    //update_enemies_death(ctx);
    //update_player_swing(ctx);

    // also "action" system update fx sword arc ect
    //action_system::update_actions(&mut ctx.ecs.actions_info, &mut ctx.ecs.physics, &mut ctx.state, delta as f32, &ctx.actions);


    // PLAYER MOVEMENT

    update_player_movement(ctx.cameras.current(), &ctx.controls, ctx.entities.player_mut(), delta);



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


fn update_player_movement(camera: &dyn camera::Camera, controls: &controls::Controls, player: &mut entity::Entity,  _delta:  f32) {
    if ! can_move(player.get_state()) {
        game::update_velocity(&mut player.physics, na::Vector3::new(0.0, 0.0, 0.0));
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

fn can_move(state: entity::EntityState) -> bool {
    match state {
        Idle => true,
        Moving => true,
        // Attacking - false
    }
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
