use crate::render_gl;
use crate::entity::*;

#[derive(Clone)]
pub struct Entity {
    pub physics: Physics,
    pub health: Health,
    pub model_name: String,
    pub animation_player: Option<render_gl::AnimationPlayer>,
    state: EntityState,

    pub bones: Vec::<na::Matrix4::<f32>>,
    pub skeleton: render_gl::Skeleton,

}


impl Entity {

    pub fn new(physics: Physics, health: Health, animation_player: Option<render_gl::AnimationPlayer>, model_name: String) -> Self {
        Entity {
            physics,
            health,
            model_name,
            animation_player,
            state: EntityState::Idle,
            bones: Vec::new(),
            skeleton: render_gl::Skeleton {
                name: "empty".to_string(),
                joints: Vec::new(),
            }
        }
    }

    pub fn get_state(&self) -> EntityState {
        self.state
    }


    pub fn update_animations(&mut self, delta: f32) {

        if let Some(animation_player) = &mut self.animation_player {
            animation_player.set_frame_bones(&mut self.bones, &mut self.skeleton, delta);
        }
    }

    pub fn update_state(&mut self, state: EntityState) {
        self.state = state;

        if let Some(animation_player) = &mut self.animation_player {

            match state {
                EntityState::Moving => animation_player.set_current(render_gl::PlayerAnimation::Walk, &self.skeleton),
                EntityState::Idle => animation_player.set_current(render_gl::PlayerAnimation::Idle, &self.skeleton),
            };
        };
    }
}

pub struct Entities {
    pub next_id: usize,
    pub player_id: usize,
    pub hammer_id: usize,
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
            hammer_id: 0,
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




    pub fn hammer(&self) -> &Entity {
        match self.entities_map.get(&self.hammer_id) {
            Some(e) => e,
            None => panic!("No hammer set")
        }
    }

    pub fn hammer_mut(&mut self) -> &mut Entity {
        match self.entities_map.get_mut(&self.hammer_id) {
            Some(e) => e,
            None => panic!("No hammer set")
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
