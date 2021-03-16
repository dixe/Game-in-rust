use nalgebra as na;

use crate::entity;



pub struct Animation {
    pub entity_id: usize,
    pub time_passed: i32,
}


impl Animation {

    pub fn new(entity_id: usize) -> Animation {

        Animation {
            time_passed: 0,
            entity_id,
        }
    }

    pub fn calculate_model_mat(&self, mut pos: na::Vector3::<f32>, rotation_sin: f32, rotation_cos: f32, scale: f32) -> na::Matrix4::<f32> {

        let scale_mat = na::Matrix4::<f32>::new(
            scale, 0.0, 0.0, 0.0,
            0.0,scale, 0.0, 0.0,
            0.0, 0.0, scale, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let rot_mat = na::Matrix4::<f32>::new(
            rotation_cos, -rotation_sin, 0.0, 0.0,
            rotation_sin, rotation_cos, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        pos.z += 0.5 * (self.time_passed as f32 / 300.0).sin();

        let trans_mat = na::Matrix4::new_translation(&pos);

        trans_mat * rot_mat * scale_mat
    }


    pub fn update(&mut self, entity: entity::Physics, delta: i32) {

        self.time_passed += delta;

    }
}
