use crate::render_gl::{KeyframeAnimation, KeyFrame, PlayerAnimations};



#[derive(Debug, Copy, Clone)]
pub enum PlayerAnimation {
    TPose,
    Walk
}


#[derive(Clone)]
pub struct AnimationPlayer {
    current_animation: PlayerAnimation,
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
        }
    }

    pub fn set_bones(&mut self, bones: Vec::<na::Matrix4::<f32>> ) {
        self.bones = bones;
    }

    pub fn set_current(&mut self, animation: PlayerAnimation) {
        self.current_animation = animation;
        self.elapsed = 0.0;
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
        };


        let frame_time = current_animation.duration/ current_animation.key_frames.len() as f32;

        // find next frame id


        let frame_index = usize::min(current_animation.key_frames.len() -1,  (self.elapsed / frame_time) as usize);


        let fi = frame_index  as f32;

        let min = frame_time * fi;
        let max = frame_time * (fi + 1.0);
        let t = clamp01(self.elapsed, min, max);



        current_animation.move_to_key_frame(&mut self.bones, frame_index, t);

        self.elapsed += delta;

        if self.elapsed > current_animation.duration {
            self.elapsed = 0.0
        }


    }
}


fn clamp01(t: f32, min: f32, max: f32) -> f32{
    f32::max(f32::min(1.0, (t - min) / (max - min)), 0.0)
}
