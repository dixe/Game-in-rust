use crate::render_gl::{KeyframeAnimation, KeyFrame, PlayerAnimations};



#[derive(Debug, Clone)]
pub enum PlayerAnimation {
    TPose,
    Walk,
    Transition(KeyframeAnimation)
}


#[derive(Clone)]
pub struct AnimationPlayer {
    current_animation: PlayerAnimation,
    next_animation: Option<PlayerAnimation>,
    elapsed: f32,
    pub player_animations: PlayerAnimations,
    pub bones: Vec::<na::Matrix4::<f32>>,
}

impl AnimationPlayer {


    pub fn new(current_animation: PlayerAnimation, player_animations: PlayerAnimations) -> Self {
        AnimationPlayer {
            current_animation,
            elapsed: 0.0,
            player_animations,
            bones: Vec::new(),
            next_animation: None,
        }
    }

    pub fn set_bones(&mut self, bones: Vec::<na::Matrix4::<f32>> ) {
        self.bones = bones;
    }

    pub fn set_current(&mut self, animation: PlayerAnimation) {
        let key_frame_end = (match animation {
            PlayerAnimation::TPose => self.player_animations.t_pose.key_frames[0].clone(),
            PlayerAnimation::Walk => self.player_animations.walk.key_frames[0].clone(),
            PlayerAnimation::Transition(ref anim) => anim.key_frames[0].clone(),
        });

        self.next_animation = Some(animation);
        self.elapsed = 0.0;

        // create transition animation

        let transition_start = self.current_key_frame();

        let transition_time = 0.2;

        let keyFrames = vec![self.current_key_frame(), key_frame_end];

        self.current_animation = PlayerAnimation::Transition( KeyframeAnimation::new("transition", transition_time, self.player_animations.t_pose.skeleton.clone(), keyFrames, false));

    }


    fn get_current_animation(&self) -> &KeyframeAnimation {

        match &self.current_animation {
            PlayerAnimation::TPose => {
                println!("loading TPose");
                &self.player_animations.t_pose
            },
            PlayerAnimation::Walk => {
                println!("loading Walk");
                &self.player_animations.walk
            },
            PlayerAnimation::Transition(ref anim) => {
                println!("loading transition");
                anim
            }
        }
    }

    pub fn current_key_frame(&self) -> KeyFrame {

        let current_animation = self.get_current_animation();

        let frame_time = current_animation.duration / current_animation.key_frames.len() as f32;
        // find next frame id
        let frame_index = usize::min(current_animation.key_frames.len() - 1,  (self.elapsed / frame_time) as usize);

        current_animation.key_frames[frame_index].clone()
    }


    pub fn set_player_animations(&mut self, animations: PlayerAnimations) {
        self.player_animations = animations;
    }



    pub fn set_frame_bones(&mut self, delta: f32) {
        // find let t =

        let current_animation = match self.current_animation {
            PlayerAnimation::TPose => {
                &mut self.player_animations.t_pose
            },
            PlayerAnimation::Walk => {
                &mut self.player_animations.walk
            },
            PlayerAnimation::Transition(ref mut anim) => anim
        };


        let frame_time = current_animation.duration/ current_animation.key_frames.len() as f32;

        // find next frame id
        // +1 to ceil, instead of floor.
        let next_frame_index = usize::min(current_animation.key_frames.len() -1,  ((self.elapsed / frame_time) + 1.0) as usize);

        let fi = match next_frame_index > 0 {
            true => next_frame_index - 1,
            false => next_frame_index
        } as f32;

        let min = frame_time * fi;
        let max = frame_time * (next_frame_index + 1) as f32;

        let t = clamp01(self.elapsed, min, max);
        println!("{} {} {:#?}", self.elapsed, min, max);


        current_animation.move_to_key_frame(&mut self.bones, next_frame_index, t);

        self.elapsed += delta;

        if self.elapsed > current_animation.duration {
            match self.next_animation {
                Some(ref next) => {
                    self.current_animation = next.clone();
                    self.next_animation = None;
                },
                _ => {}
            };

            self.elapsed = 0.0

        }


    }
}


fn clamp01(t: f32, min: f32, max: f32) -> f32{
    f32::max(f32::min(1.0, (t - min) / (max - min)), 0.0)
}
