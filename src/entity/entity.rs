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
            weapon_id: 9999999,
            hit_boxes: Vec::<physics::CollisionBox>::new(),
            is_hit: false,
            queued_action: None
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


    pub fn next_action(&mut self) {

        match self.queued_action {
            Some(action) => {
                self.update_state(action);
                self.queued_action = None;
            },
            None => {
            }
        };
    }

    fn update_state(&mut self, state: EntityState) {

        self.state = state;

        if let Some(animation_player) = &mut self.animation_player {

            match state {
                EntityState::Moving => animation_player.set_current(render_gl::Animation::Walk, &self.skeleton),
                EntityState::Attack(info) => {
                    if info.combo_num == 1 {
                        animation_player.set_current(render_gl::Animation::AttackFollow, &self.skeleton)
                    }
                    else {
                        animation_player.set_current(render_gl::Animation::Attack, &self.skeleton)
                    }
                },
                EntityState::Idle => animation_player.set_current(render_gl::Animation::Idle, &self.skeleton),
                EntityState::Roll => animation_player.set_current(render_gl::Animation::Roll, &self.skeleton),
            };
        };
    }
}
