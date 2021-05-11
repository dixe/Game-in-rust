use crate::game;
use crate::physics;
use crate::entity;

use crate::controls;
use crate::camera;
use crate::render_gl;


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

pub fn update_game_state(ctx: &mut game::Context, _collisions: &Vec<physics::EntityCollision>) {

    // also "action" system update fx sword arc ect
    //action_system::update_actions(&mut ctx.ecs.actions_info, &mut ctx.ecs.physics, &mut ctx.state, delta as f32, &ctx.actions);


    // PLAYER MOVEMENT
    update_player(ctx.cameras.current(), &ctx.controls, &mut ctx.entities.player, &ctx.entities.weapons, &ctx.animations);


    // make a function on player, weapon anchor mat and just use that as world_matrix
    // or a function to pass in the hammer reference, even though it might
    // not play nice with borrow checker


    let world_mat = ctx.entities.player.skeleton.joints[14].world_matrix;
    let player_model_mat = ctx.entities.player.physics.calculate_model_mat();

    let player = &ctx.entities.player;

    let weapon = match ctx.entities.weapons.get_mut(player.weapon_id) {
        Some(weapon) => weapon,
        None => &mut ctx.entities.default_weapon // default weapon for player
    };

    weapon.physics.apply_transform(player_model_mat * world_mat);
    let player = &ctx.entities.player;


    // Weapon collisions
    // only if player is attacking and attack animation is in attack state
    if let entity::EntityState::Attack(info) = player.get_state()  {

        let current_frame = player.animation_player.as_ref().unwrap().current_frame_number();

        if current_frame >= info.hit_start_frame && current_frame <= info.hit_end_frame {

            for dummy in ctx.entities.enemies.values_mut() {
                dummy.is_hit = false;
                if entity_collision(&weapon, dummy) {
                    resolve_player_hit_enemy(player, dummy);
                    dummy.is_hit = true;
                }
            }
        }
    }
}


fn resolve_player_hit_enemy(_player: &entity::Entity, _enemy: &mut entity::Entity) {


}

//TODO move this into physics and call into that one
fn entity_collision(entity_1: &entity::Entity, entity_2: &entity::Entity) -> bool{

    // TODO make this more optimized, by calculation each transformed hitbox only once
    for e1_hitbox_base in &entity_1.hit_boxes {
        let e1_hitbox = e1_hitbox_base.make_transformed(entity_1.physics.pos, entity_1.physics.rotation);

        for e2_hitbox_base in &entity_2.hit_boxes {
            let e2_hitbox = e2_hitbox_base.make_transformed(entity_2.physics.pos, entity_2.physics.rotation);
            let collision_res = physics::check_collision(&e1_hitbox, &e2_hitbox);
            if collision_res.has_collision() {
                return true;
            }

        }
    }

    false
}



fn update_player(camera: &dyn camera::Camera, controls: &controls::Controls, player: &mut entity::Entity, weapons: &entity::EntitiesCollection, animations: &std::collections::HashMap<String, render_gl::PlayerAnimations>) {

    // UPDATE STATE, IE WHEN ATTACK IS DONE SET BACK TO IDLE
    update_player_state(player);

    if controls.roll {
        perform_roll(player);
        return;
    }

    if controls.attack {
        //TODO get attack start and end frame from player/current weapon
        perform_attack(player);
        return;
    }

    if !can_perform_action(player.get_state()) {
        game::update_velocity(&mut player.physics, na::Vector3::new(0.0, 0.0, 0.0));
        return;
    }


    if controls.next_weapon {
        // Also change the animations on the animation player
        player.weapon_id = (player.weapon_id + 1) % (weapons.count() + 1);


        let new_weapon_name = match weapons.get(player.weapon_id) {
            Some(w) => &w.model_name,
            None => &player.model_name,
        };

        let new_animations = animations.get(new_weapon_name).unwrap();

        player.animation_player.as_mut().unwrap().set_animations(new_animations.clone());
        println!("Weapon {:#?}", new_weapon_name);

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
                player.queued_action = Some(target_state);
            }
        },

        camera::CameraMode::Free => {},
    }
}


fn perform_roll(entity: &mut entity::Entity) {
    if can_perform_action(entity.get_state()) {
        entity.queued_action = Some(entity::EntityState::Roll);
    }
}


fn perform_attack(entity: &mut entity::Entity) {
    match entity.get_state() {
        entity::EntityState::Attack(info) => {

            let current_frame = entity.animation_player.as_ref().unwrap().current_frame_number();

            // this is to not buffer for too long, otherwise it feels as if
            // the attack was not indented
            if current_frame >= info.hit_start_frame {

                let attack_info = entity::AttackInfo {
                    combo_num: 1 - info.combo_num,
                    hit_start_frame: 9,
                    hit_end_frame: 20,
                };

                entity.queued_action = Some(entity::EntityState::Attack(attack_info));
            }
        },
        _ => {
            let attack_info = entity::AttackInfo {
                combo_num: 0,
                hit_start_frame: 9,
                hit_end_frame: 20,
            };

            entity.queued_action = Some(entity::EntityState::Attack(attack_info));
        }
    };
}

fn can_perform_action(state: entity::EntityState) -> bool {
    match state {
        entity::EntityState::Idle => true,
        entity::EntityState::Moving => true,
        entity::EntityState::Attack(_) => false,
        entity::EntityState::Roll => false,
    }
}


fn update_player_state(player: &mut entity::Entity) {
    let mut next_action = false;
    match player.get_state() {
        entity::EntityState::Attack(info) => {
            // CHECK IF WE ARE IN COMBO FRAME RANGE
            let current_frame = player.animation_player.as_ref().unwrap().current_frame_number();
            next_action |= current_frame >= info.hit_end_frame;

            if player.animation_player.as_ref().unwrap().has_repeated {
                next_action = true;
                if player.queued_action == None {
                    player.queued_action = Some(entity::EntityState::Idle);
                }
                // check if animation is don
            }
        },
        entity::EntityState::Roll => {
            next_action = player.animation_player.as_ref().unwrap().has_repeated;
            if player.queued_action == None {
                player.queued_action = Some(entity::EntityState::Idle);
            }
        },
        _ => {
            next_action = true;
        }
    };

    if next_action {
        player.next_action();
    }
}



fn update_enemies_death(_ctx: &mut game::Context) {
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
