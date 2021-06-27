use crate::base_entity::*;
use crate::behaviours::*;


pub struct AiRunData<'a> {
    pub entity: &'a mut BaseEntity,
    pub player: &'a BaseEntity,
}


pub trait Ai<T> {
    fn run(&self, run_data: AiRunData, ai_data: &mut T);
}


#[derive(Clone)]
pub struct RegularEnemyState {

    pub distance: f32,
    pub current_behaviour: Behaviour
}


#[derive(Clone)]
pub enum EntityAi {
    RegularEnemy(RegularEnemyState),
    BossEnemy
}

impl EntityAi {

    pub fn regular_enemy(distance: f32) -> Self {
        let data =
            RegularEnemyState {
                distance,
                current_behaviour: Behaviour::Empty

            };
        EntityAi::RegularEnemy(data)
    }
}
