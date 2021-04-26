use crate::render_gl;
use crate::entity::*;

#[derive(Clone)]
pub struct Entity {
    pub physics: Physics,
    pub health: Health,
    pub model_id: usize,
    pub animation_player: render_gl::AnimationPlayer,
    state: EntityState,
}


impl Entity {

    pub fn new(physics: Physics, health: Health, animation_player:render_gl::AnimationPlayer, model_id: usize) -> Self {
        Entity {
            physics,
            health,
            model_id,
            animation_player,
            state: EntityState::Idle,
        }
    }

    pub fn get_state(&self) -> EntityState {
        self.state
    }

    pub fn update_state(&mut self, state: EntityState) {
        self.state = state;

        match state {
            EntityState::Moving => self.animation_player.set_current(render_gl::PlayerAnimation::Walk),
            EntityState::Idle =>  self.animation_player.set_current(render_gl::PlayerAnimation::TPose),
        };
    }
}

pub struct Entities {
    pub next_id: usize,
    pub player_id: usize,
    pub entities_map: std::collections::HashMap::<usize, Entity>,

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EntityState {
    Idle,
    Moving,
}

impl Entities {


    pub fn new() -> Self {
        Entities {
            next_id: 0,
            player_id: 0,
            entities_map: std::collections::HashMap::<usize, Entity>::new(),
        }
    }

    pub fn player(&self) -> &Entity {
        match self.entities_map.get(&self.player_id) {
            Some(e) => e,
            None => panic!("No player set")
        }
    }

    pub fn player_mut(&mut self) -> &mut Entity {
        match self.entities_map.get_mut(&self.player_id) {
            Some(e) => e,
            None => panic!("No player set")
        }
    }

    pub fn get(&self, entity_id: usize) -> Option<&Entity> {
        self.entities_map.get(&entity_id)
    }

    pub fn add(&mut self, entity: Entity) -> usize {

        let id = self.next_id;
        self.next_id += 1;


        self.entities_map.insert(id, entity);
        id
    }

    pub fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, usize, Entity> {
        self.entities_map.values_mut()
    }


    pub fn set_physics(&mut self, entity_id: usize, physics: entity::Physics) {

        match self.entities_map.get_mut(&entity_id) {
            Some(e) => {
                e.physics = physics  }
            _ => {}
        };
    }

}
