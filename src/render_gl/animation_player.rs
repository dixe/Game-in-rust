use crate::render_gl::{KeyframeAnimation, KeyFrame};


pub struct AnimationPlayer {
    current_animation: KeyframeAnimation,
    elapsed: f32
}

impl AnimationPlayer {


    pub fn new(current_animation: KeyframeAnimation,) -> Self {
        AnimationPlayer {
            current_animation,
            elapsed: 0.0
        }
    }


    pub fn current_key_frame(&self) -> KeyFrame {
        let frame_time = self.current_animation.duration/ self.current_animation.key_frames.len() as f32;
        // find next frame id
        let frame_index = usize::min(self.current_animation.key_frames.len() -1,  (self.elapsed / frame_time) as usize);

        self.current_animation.key_frames[frame_index].clone()
    }


    pub fn set_frame_bones(&mut self, bones: &mut [na::Matrix4::<f32>], delta: f32) {

        // find let t =

        let frame_time = self.current_animation.duration/ self.current_animation.key_frames.len() as f32;

        // find next frame id


        let frame_index = usize::min(self.current_animation.key_frames.len() -1,  (self.elapsed / frame_time) as usize);


        let fi = frame_index  as f32;

        let min = frame_time * fi;
        let max = frame_time * (fi + 1.0);
        let t = clamp01(self.elapsed, min, max);

        //println!("t el min, max frame_time {}  {} {} {} {}", t, self.elapsed, min, max, frame_time);


        self.current_animation.move_to_key_frame(bones, frame_index, t);

        self.elapsed += delta;

        if self.elapsed > self.current_animation.duration {
            self.elapsed = 0.0
        }


    }
}


fn clamp01(t: f32, min: f32, max: f32) -> f32{
    f32::max(f32::min(1.0, (t - min) / (max - min)), 0.0)
}
