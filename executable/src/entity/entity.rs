use crate::render_gl;
use crate::entity::*;
use crate::physics;
use crate::shared;


#[derive(Clone)]
pub struct Entity {
    pub id: usize,
    pub base_entity: shared::BaseEntity,
    pub model_name: String,
    pub animation_player: Option<render_gl::AnimationPlayer>,
    pub bones: Vec::<na::Matrix4::<f32>>,
    pub skeleton: render_gl::Skeleton,
    pub hitboxes: Vec::<physics::CollisionBox>,
    pub weapon_id: usize,
    pub is_hit: bool,

}


impl Entity {

    pub fn new(animation_player: Option<render_gl::AnimationPlayer>, model_name: String) -> Self {
        Entity {
            base_entity: shared::BaseEntity::new(),
            model_name,
            animation_player,
            bones: Vec::new(),
            skeleton: render_gl::Skeleton {
                name: "empty".to_string(),
                joints: Vec::new(),
                legs: None,
            },
            weapon_id: 9999999,
            hitboxes: Vec::<physics::CollisionBox>::new(),
            is_hit: false,
            id: 0
        }
    }

    pub fn get_state(&self) -> shared::EntityState {
        self.base_entity.state
    }


    pub fn update_animations(&mut self, delta: f32) {
        if let Some(animation_player) = &mut self.animation_player {
            animation_player.update_skeleton(&mut self.skeleton, delta);
        }

        render_gl::inverse_kinematics::update_ik(&mut self.skeleton, &self.base_entity.physics, delta);

        self.skeleton.set_bones_from_skeleton(&mut self.bones);

    }


    pub fn next_action(&mut self) {

        match self.base_entity.queued_action {
            Some(action) => {
                self.update_state(action);
                self.base_entity.queued_action = None;
            },
            None => {
            }
        };
    }

    fn update_state(&mut self, state: shared::EntityState) {

        self.base_entity.state = state;

        if let Some(animation_player) = &mut self.animation_player {

            match state {
                shared::EntityState::Moving => animation_player.set_current(render_gl::Animation::Walk, &self.skeleton),
                shared::EntityState::Attack(info) => {
                    if info.combo_num == 1 {
                        animation_player.set_current(render_gl::Animation::AttackFollow, &self.skeleton)
                    }
                    else {
                        animation_player.set_current(render_gl::Animation::Attack, &self.skeleton)
                    }
                },
                shared::EntityState::Idle => animation_player.set_current(render_gl::Animation::Idle, &self.skeleton),
                shared::EntityState::Roll => animation_player.set_current(render_gl::Animation::Roll, &self.skeleton),
            };
        };
    }
}
