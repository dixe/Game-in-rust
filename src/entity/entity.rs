use crate::render_gl;
use crate::entity::*;
use crate::physics;

#[derive(Clone)]
pub struct Entity {
    pub physics: Physics,
    pub health: Health,
    pub model_name: String,
    pub animation_player: Option<render_gl::AnimationPlayer>,
    state: EntityState,
    pub bones: Vec::<na::Matrix4::<f32>>,
    pub skeleton: render_gl::Skeleton,
    pub hit_boxes: Vec::<physics::CollisionBox>,
    pub weapon_id: usize,
    pub is_hit: bool
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EntityState {
    Idle,
    Moving,
    Attack(usize, usize),
}

impl Entity {

    pub fn new(animation_player: Option<render_gl::AnimationPlayer>, model_name: String) -> Self {
        Entity {
            physics: Physics::new(),
            health: Health::new(100.0),
            model_name,
            animation_player,
            state: EntityState::Idle,
            bones: Vec::new(),
            skeleton: render_gl::Skeleton {
                name: "empty".to_string(),
                joints: Vec::new(),
            },
            weapon_id: 0,
            hit_boxes: Vec::<physics::CollisionBox>::new(),
            is_hit: false,
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
                EntityState::Attack(_,_) => animation_player.set_current(render_gl::PlayerAnimation::Attack, &self.skeleton),
                EntityState::Idle => animation_player.set_current(render_gl::PlayerAnimation::Idle, &self.skeleton),
            };
        };
    }
}
