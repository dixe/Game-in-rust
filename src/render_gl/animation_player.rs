use crate::render_gl::{KeyframeAnimation, KeyFrame, Skeleton, PlayerAnimations};



#[derive(Debug, Clone)]
pub enum Animation {
    TPose,
    Idle,
    Walk,
    Attack,
    AttackFollow,
    Transition(KeyframeAnimation)
}

#[derive(Clone)]
pub struct AnimationPlayer {
    current_animation: Animation,
    next_animation: Option<Animation>,
    elapsed: f32,
    pub has_repeated: bool,
    pub animations: PlayerAnimations,
}

impl AnimationPlayer {

    pub fn new(current_animation: Animation, animations: PlayerAnimations) -> Self {
        AnimationPlayer {
            current_animation,
            elapsed: 0.0,
            animations,
            has_repeated: false,
            next_animation: None,
        }
    }

    pub fn set_current(&mut self, animation: Animation, skeleton: &Skeleton) {
        let should_transition = match animation {
            Animation::Attack => false,
            _ => true,
        };

        if should_transition {
            self.transition_into_next(animation, skeleton);
        }
        else {
            self.current_animation = animation;
            self.next_animation = None;
            self.has_repeated = false;
            self.elapsed = 0.0;
        }
    }


    pub fn current_frame_number(&self) -> usize {
        let current_animation = self.current_animation();
        let frame_time = current_animation.duration / current_animation.key_frames.len() as f32;


        usize::min(current_animation.key_frames.len() - 1,  (self.elapsed / frame_time) as usize)

    }

    pub fn set_animations(&mut self, animations: PlayerAnimations) {
        self.animations = animations;
    }

    pub fn set_frame_bones(&mut self, bones: &mut Vec::<na::Matrix4::<f32>>, skeleton: &mut Skeleton, delta: f32) {

        let (t, next_frame_index) = self.current_t();

        self.elapsed += delta;

        let current_animation = match self.current_animation {
            Animation::TPose => {
                &mut self.animations.t_pose
            },
            Animation::Walk => {
                &mut self.animations.walk
            },
            Animation::Idle => {
                &mut self.animations.idle
            },
            Animation::Attack => {
                &mut self.animations.attack
            },
            Animation::AttackFollow => {
                &mut self.animations.attack_follow
            },
            Animation::Transition(ref mut anim) => anim
        };

        current_animation.move_to_key_frame(bones, skeleton, next_frame_index, t);

        if self.elapsed > current_animation.duration {
            match self.next_animation {
                Some(ref next) => {
                    self.current_animation = next.clone();
                    self.next_animation = None;
                    self.has_repeated = false;
                },
                _ => {
                    self.has_repeated = true;
                }
            };

            if self.is_current_cyclic() {
                self.elapsed = 0.0;
            }
        }
    }




    fn transition_into_next(&mut self, animation: Animation, skeleton: &Skeleton) {
        let next_start_key_frame = match animation {
            Animation::TPose => self.animations.t_pose.key_frames[0].clone(),
            Animation::Idle => self.animations.idle.key_frames[0].clone(),
            Animation::Walk => self.animations.walk.key_frames[0].clone(),
            Animation::Attack => self.animations.attack.key_frames[0].clone(),
            Animation::AttackFollow => self.animations.attack_follow.key_frames[0].clone(),
            Animation::Transition(ref anim) => anim.key_frames[0].clone(),
        };

        self.next_animation = Some(animation);


        // create transition animation from current frame state
        let transition_time = 0.2;

        let keyFrames = vec![self.current_frame(skeleton), next_start_key_frame];
        // important that this is after we call current_frame, since that uses the elapsed time
        self.elapsed = 0.0;


        self.current_animation = Animation::Transition(KeyframeAnimation::new(transition_time, keyFrames, false));
        self.has_repeated = false;
    }


    fn current_animation(&self) -> &KeyframeAnimation {

        match &self.current_animation {
            Animation::TPose => {
                &self.animations.t_pose
            },
            Animation::Walk => {
                &self.animations.walk
            },
            Animation::Idle => {
                &self.animations.idle
            },
            Animation::Attack => {
                &self.animations.attack
            },
            Animation::AttackFollow => {
                &self.animations.attack_follow
            },
            Animation::Transition(ref anim) => {
                anim
            }
        }
    }


    fn current_frame(&self, skeleton: &Skeleton) -> KeyFrame {

        let current_animation = self.current_animation();

        let frame_time = current_animation.duration / current_animation.key_frames.len() as f32;

        // find current frame id
        let _frame_index = usize::min(current_animation.key_frames.len() - 1,  (self.elapsed / frame_time) as usize);

        let (t, next_frame_index) = self.current_t();

        current_animation.keyframe_from_t(skeleton, next_frame_index, t)

    }




    fn current_t(&self) -> (f32, usize) {
        let current_animation = self.current_animation();

        let frame_time = current_animation.duration / current_animation.key_frames.len() as f32;

        // find next frame id
        // +1 to ceil, instead of floor.
        let next_frame_index = usize::min(current_animation.key_frames.len() -1,  ((self.elapsed / frame_time) + 1.0) as usize);

        let fi = match next_frame_index > 0 {
            true => next_frame_index - 1,
            false => next_frame_index
        } as f32;

        let min = frame_time * fi;
        let max = frame_time * (next_frame_index + 1) as f32;

        (clamp01(self.elapsed, min, max), next_frame_index)
    }


    fn is_current_cyclic(&self) -> bool {
        match &self.current_animation {
            Animation::TPose => {
                self.animations.t_pose.cyclic
            },
            Animation::Walk => {
                self.animations.walk.cyclic
            },
            Animation::Idle => {
                self.animations.idle.cyclic
            },
            Animation::Attack => {
                self.animations.attack.cyclic
            },

            Animation::AttackFollow => {
                self.animations.attack.cyclic
            },
            Animation::Transition(_) => {
                false
            }
        }

    }




}




fn clamp01(t: f32, min: f32, max: f32) -> f32{
    f32::max(f32::min(1.0, (t - min) / (max - min)), 0.0)
}
