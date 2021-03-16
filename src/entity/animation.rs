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

    pub fn calculate_model_mat(&self, physics: &entity::Physics) -> na::Matrix4::<f32> {

        let scale_mat = na::Matrix4::<f32>::new(
            physics.scale, 0.0, 0.0, 0.0,
            0.0, physics.scale, 0.0, 0.0,
            0.0, 0.0, physics.scale, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let mut pos = physics.pos;

        let rot_mat = na::Matrix4::<f32>::new_rotation(physics.rotation);

        pos.z += 0.5 * (self.time_passed as f32 / 300.0).sin();

        let trans_mat = na::Matrix4::new_translation(&pos);

        trans_mat * rot_mat * scale_mat
    }


    pub fn update(&mut self, entity: entity::Physics, delta: i32) {

        self.time_passed += delta;

    }
}
