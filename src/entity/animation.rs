use nalgebra as na;

use crate::render_gl;
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

    pub fn calculate_model_mat(&self, physics: &entity::Physics, anchor_physics: Option<&entity::Physics>) -> na::Matrix4::<f32> {

        let mut pos = physics.pos;

        pos.z += 0.5 * (self.time_passed as f32 / 300.0).sin();

        let mut model_mat = render_gl::calculate_model_mat(physics, anchor_physics);


        match anchor_physics {
            Some(_) => println!("Has Anchor"),
            _ => {}
        }

        match anchor_physics {
            Some(anchor) => {
                let anchor_trans = na::Matrix4::new_translation(&anchor.pos) ;
                model_mat = anchor_trans * model_mat;
                println!("With anchor");
            },
            _ => {}
        }
        model_mat
    }


    pub fn update(&mut self, entity: entity::Physics, delta: i32) {

        self.time_passed += delta;

    }
}
