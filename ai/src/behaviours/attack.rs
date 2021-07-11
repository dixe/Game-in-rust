use shared::*;
use nalgebra as na;

use crate::behaviours::*;


#[derive(PartialEq)]
pub enum AttackResult {
    ClosingDistance,
    Attacking
}


pub fn attack(entity: &mut BaseEntity, target: &BaseEntity) -> AttackResult {


    //TODO get attack range, from weapon

    let target_distance = (entity.physics.pos - target.physics.pos).magnitude();


    // TODO also get this from weapon
    let attack_distance = 2.0;

    if target_distance > attack_distance {
        keep_distance(0.0, entity, target);
        return AttackResult::ClosingDistance;
    }


    // Distance closed attack


    // TODO Get this from somewhere, maybe have an actions that is just attack and then figure it out based on the
    // current weapon. Or even better have this info on the weapons and then just call into them
    let attack_info = shared::AttackInfo {
        combo_num: 0,
        hit_start_frame: 9,
        hit_end_frame: 20,
    };

    entity.queued_action = Some(EntityState::Attack(attack_info));
    return AttackResult::Attacking;

}
