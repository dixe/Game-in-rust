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

pub fn update_game_state(scene: &mut game::Scene, controls: &controls::Controls,  _collisions: &Vec<physics::EntityCollision>) {

    // also "action" system update fx sword arc ect
    //action_system::update_actions(&mut scene.ecs.actions_info, &mut scene.ecs.physics, &mut scene.state, delta as f32, &scene.actions);


    // MOVEMENT AND STATES
    update_player(scene.cameras.current(), controls, &mut scene.entities.player, &scene.entities.weapons, &scene.animations);
    update_enemies(scene);


    // WEAPONS TRANSFORMS AND COLLISIONS
    let player = &mut scene.entities.player;
    update_entity_weapon_and_collisions(player, &mut scene.entities.weapons, &mut scene.entities.enemies.values_mut());


}

fn update_entity_weapon_and_collisions<'a, I>(entity: &mut entity::Entity, weapons: &mut entity::EntitiesCollection, targets: I) where
    I: Iterator<Item = &'a mut entity::Entity> {
    let weapon = match weapons.get_mut(entity.weapon_id) {
        Some(weapon) => weapon,
        None => {
            return ;
        }
    };

    let world_mat = entity.skeleton.joints[14].world_matrix;
    let model_mat = entity.base_entity.physics.calculate_model_mat();

    weapon.base_entity.physics.apply_transform(model_mat * world_mat);


    if let shared::EntityState::Attack(info) = entity.get_state() {

        let current_frame = entity.animation_player.as_ref().unwrap().current_frame_number();

        if current_frame >= info.hit_start_frame && current_frame <= info.hit_end_frame {

            for target in targets {
                target.is_hit = false;
                if entity_collision(&weapon, target) {
                    resolve_player_hit_enemy(entity, target);
                    target.is_hit = true;
                }

            }
        }
    }

}



fn update_enemies(scene: &mut game::Scene) {
    update_enemies_states(scene);
}


fn resolve_player_hit_enemy(_player: &entity::Entity, _enemy: &mut entity::Entity) {


}

//TODO move this into physics and call into that one
fn entity_collision(entity_1: &entity::Entity, entity_2: &entity::Entity) -> bool{

    // TODO make this more optimized, by calculation each transformed hitbox only once
    for e1_hitbox_base in &entity_1.hitboxes {
        let e1_hitbox = e1_hitbox_base.make_transformed(entity_1.base_entity.physics.pos, entity_1.base_entity.physics.rotation);

        for e2_hitbox_base in &entity_2.hitboxes {
            let e2_hitbox = e2_hitbox_base.make_transformed(entity_2.base_entity.physics.pos, entity_2.base_entity.physics.rotation);
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
    update_entity_state(player);

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
        shared::physics_functions::update_velocity(&mut player.base_entity.physics, na::Vector3::new(0.0, 0.0, 0.0));
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
            let y = -controls.movement_dir.y;
            let x = controls.movement_dir.x;
            let player_move_dir = rot_mat * na::Vector3::new(-controls.movement_dir.y, controls.movement_dir.x, 0.0);

            shared::physics_functions::update_velocity(&mut player.base_entity.physics, player_move_dir);

            if player_move_dir.magnitude() > 0.0 {
                player.base_entity.physics.facing_dir = player_move_dir.normalize();
            }

        },

        camera::CameraMode::Free => {
            return;
            let player_move_dir = na::Vector3::new(controls.movement_dir.y, 0.0, 0.0);

            shared::physics_functions::update_velocity(&mut player.base_entity.physics, player_move_dir);
        },
    }
}


fn perform_roll(entity: &mut entity::Entity) {
    if can_perform_action(entity.get_state()) {
        entity.base_entity.queued_action = Some(shared::EntityState::Roll);
    }
}


fn perform_attack(entity: &mut entity::Entity) {
    match entity.get_state() {
        shared::EntityState::Attack(info) => {

            let current_frame = entity.animation_player.as_ref().unwrap().current_frame_number();

            let attack_info = shared::AttackInfo {
                combo_num: 1 - info.combo_num,
                hit_start_frame: 9,
                hit_end_frame: 20,
            };

            entity.base_entity.queued_action = Some(shared::EntityState::Attack(attack_info));

        },
        _ => {
            let attack_info = shared::AttackInfo {
                combo_num: 0,
                hit_start_frame: 9,
                hit_end_frame: 20,
            };

            entity.base_entity.queued_action = Some(shared::EntityState::Attack(attack_info));
        }
    };
}

fn can_perform_action(state: shared::EntityState) -> bool {
    match state {
        shared::EntityState::Idle => true,
        shared::EntityState::Moving => true,
        shared::EntityState::Attack(_) => false,
        shared::EntityState::Roll => false,
    }
}


fn update_entity_state(entity: &mut entity::Entity) {

    let mut target_state = shared::EntityState::Idle;

    if entity.base_entity.physics.velocity.magnitude() > 0.0 {
        target_state = shared::EntityState::Moving;
    }

    if entity.get_state() != target_state {
        entity.base_entity.queued_action = Some(target_state);
    }

    let mut next_action = false;
    match entity.get_state() {
        shared::EntityState::Attack(info) => {
            // CHECK IF WE ARE IN COMBO FRAME RANGE
            let current_frame = entity.animation_player.as_ref().unwrap().current_frame_number();
            next_action |= current_frame >= info.hit_end_frame;

            if entity.animation_player.as_ref().unwrap().has_repeated {
                next_action = true;
                if entity.base_entity.queued_action == None {
                    entity.base_entity.queued_action = Some(shared::EntityState::Idle);
                }

            }
        },
        shared::EntityState::Roll => {
            next_action = entity.animation_player.as_ref().unwrap().has_repeated;
            if entity.base_entity.queued_action == None {
                entity.base_entity.queued_action = Some(shared::EntityState::Idle);
            }
        },
        _ => {
            next_action = true;
        }
    };

    if next_action {
        entity.next_action();
    }
}


fn update_enemies_states(scene: &mut game::Scene) {

    for enemy in scene.entities.enemies.values_mut() {
        update_entity_state(enemy);
    }
}


fn update_enemies_death(_scene: &mut game::Scene) {
    /*
    let mut deaths =  Vec::new();
    for e in &scene.state.enemies {
    let enemy_hp = match scene.ecs.get_health(*e) {
    Some(e_hp) => *e_hp,
    None => continue,
};

    if enemy_hp.health() <= 0.0 {
    deaths.push(e);
}
}

    for dead in deaths {
    scene.ecs.remove_entity(*dead);
}
     */
}
