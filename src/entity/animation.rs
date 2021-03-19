use nalgebra as na;

use crate::render_gl;
use crate::entity;



pub struct AnimationData {
    pub entity_id: usize,
    pub time_passed: i32,
    update_fn: fn(time_passed: f32, physics: &mut entity::Physics),
}


impl AnimationData {

    pub fn new(entity_id: usize, update_fn: fn(time_passed: f32, physics: &mut entity::Physics)) -> AnimationData {

        AnimationData {
            time_passed: 0,
            entity_id,
            update_fn
        }
    }

    pub fn calculate_model_mat(&self, mut physics: entity::Physics, anchor_physics: Option<&entity::Physics>) -> na::Matrix4::<f32> {


        let mut model_mat = render_gl::calculate_model_mat(&physics, anchor_physics);

        model_mat
    }

    pub fn update(&mut self, physics: &mut entity::Physics, delta: i32) {

        self.time_passed += delta;

        (self.update_fn)(self.time_passed as f32, physics);
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
