use crate::base_entity::*;


#[derive(Clone)]
pub struct BaseEntity {
    pub physics: Physics,
    pub health: Health,
    pub state: EntityState,
    pub is_hit: bool,
    pub queued_action: Option<EntityState>,

}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AttackInfo {
    pub combo_num: usize,
    pub hit_start_frame: usize,
    pub hit_end_frame: usize,

}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EntityState {
    Idle,
    Moving,
    Attack(AttackInfo),
    Roll,
}

impl BaseEntity {

    pub fn new() -> Self {
        BaseEntity {
            physics: Physics::new(),
            health: Health::new(100.0),
            state: EntityState::Idle,


            is_hit: false,
            queued_action: None,
        }
    }

}
