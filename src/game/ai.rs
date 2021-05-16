use crate::game;
use crate::entity;

use nalgebra_glm as glm;

pub fn run_ais(scene: &mut game::Scene) {

    let player = &scene.entities.player;

    for enemy in scene.entities.enemies.values_mut() {

        // keep distance of 3 to player


        match enemy.get_state() {
            entity::EntityState::Moving => {
                let player_dist_3 = (player.physics.pos - enemy.physics.pos).normalize() * 3.0;
                let to_player = player.physics.pos - player_dist_3 ;

                move_to_point(enemy, to_player);

            },
            entity::EntityState::Idle => {

                enemy.queued_action = Some(entity::EntityState::Moving);
            },
            _ => {}
        };
    };
}


fn move_to_point(entity: &mut entity::Entity, new_point: na::Vector3<f32>) {


    //TODO: Maybe remove the Z component, since for movement it is not used.
    // Unlesss entity is falling and we need to wait until it has fallen?
    // Error is that when stading on ground we are not a z = 0.0, but higer.
    // maybe also remove the rotation aspect, since walking sideways and backwards

    let mut move_vec = new_point - entity.physics.pos;

    let move_mag = move_vec.magnitude();

    let move_vec_xy = move_vec.xy();
    let move_mag_xy = move_vec_xy.magnitude();

    if move_mag_xy < 0.01 {
        /*
        // We have moved to the point
        // Attack
        let attack_info = entity::AttackInfo {
        combo_num: 1,
        hit_start_frame: 9,
        hit_end_frame: 20,
    };

        //entity.queued_action = Some(entity::EntityState::Attack(attack_info));

         */

        entity.queued_action = Some(entity::EntityState::Idle);

        game::set_velocity(&mut entity.physics, na::Vector3::new(0.0, 0.0, 0.0));
        return;
    }

    // maybe move this to set/update_velocity and have a min speed. In this case 1.0
    if move_mag < 1.0 {
        move_vec = move_vec.normalize();
    }

    game::set_velocity(&mut entity.physics, move_vec);
    game::update_rotation(&mut entity.physics, move_vec);

}



#[cfg(test)]
mod tests {






}
