use nalgebra as na;

use crate::render_gl;
use crate::entity;




#[derive(Clone)]
pub struct AnimationsInfo {
    pub entity_id: usize,
    pub default: Option<AnimationData>,
    pub queue: std::collections::VecDeque<AnimationData>,
    pub active: Option<AnimationData>
}




impl AnimationsInfo {

    pub fn new(entity_id: usize, default: Option<AnimationData>) -> AnimationsInfo {
        AnimationsInfo {
            entity_id,
            default,
            queue: std::collections::VecDeque::new(),
            active: default

        }
    }

    fn next(&mut self) {
        match self.active {
            Some(data) => {
                if data.time_passed < data.total_time {
                    return
                }

            },
            _ => {}
        };

        // if we are here active was none, or it is expired
        // set next in queue, or default

        self.active = match self.queue.pop_front() {
            None => self.default,
            data => data
        };
    }

    pub fn update(&mut self, physics: &mut entity::Physics, delta: f32) {

        match self.active {
            Some(mut data) => {
                data.update(physics, delta);
                self.active = Some(data);
            },
            _ => {},
        }



        self.next();
    }


}

#[derive(Copy, Clone)]
pub struct AnimationData {
    pub time_passed: f32,
    pub total_time: f32,
    update_fn: fn(time_passed: f32, physics: &mut entity::Physics),
}


impl AnimationData {

    pub fn new(update_fn: fn(time_passed: f32, physics: &mut entity::Physics)) -> AnimationData {

        AnimationData {
            time_passed: 0.0,
            total_time: 1.0,
            update_fn
        }
    }

    pub fn update(&mut self, physics: &mut entity::Physics, delta: f32) {
        self.time_passed += delta;

        let t = self.time_passed.clamp(0.0, self.total_time);

        (self.update_fn)(t, physics);

    }
}




/*
impl Animation for AnimationData {

fn update(&mut self, mut physics: entity::Physics, delta: i32) {

self.time_passed += delta;

physics.pos.z += 0.5 * (self.time_passed as f32 / 300.0).sin();
}

    fn entity_id(&self) -> usize {
    self.entity_id
}

}

    pub trait Animation {

    fn update(&mut self, physics: entity::Physics, delta: i32);

    fn entity_id(&self) -> usize;

    fn calculate_model_mat(&self, physics: entity::Physics, anchor_physics: Option<&entity::Physics>) -> na::Matrix4::<f32> {
    render_gl::calculate_model_mat(&physics, anchor_physics)
}
}
     */
