use nalgebra as na;

use crate::cube;
use crate::render_gl;
use crate::entity;




enum ModelType {

    Cube(cube::Cube),
    WaveModel(render_gl::Model)
}

pub struct Model {
    model: ModelType
}


impl Model {

    pub fn cube(cube: cube::Cube) -> Self {
        let model = ModelType::Cube(cube);
        Model {
            model
        }
    }

    pub fn wave_model(wave: render_gl::Model) -> Self {
        let model = ModelType::WaveModel(wave);
        Model {
            model
        }
    }

    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, physics: entity::Physics) {
        let scale_mat = na::Matrix4::<f32>::new(
            physics.scale, 0.0, 0.0, 0.0,
            0.0, physics.scale, 0.0, 0.0,
            0.0, 0.0, physics.scale, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let rot_mat = na::Matrix4::<f32>::new_rotation(physics.rotation);


        let trans_mat = na::Matrix4::new_translation(&physics.pos);

        match &self.model {
            ModelType::Cube(c) => {
                c.render(gl, shader, trans_mat * rot_mat * scale_mat);
            },
            ModelType::WaveModel(wm) => {
                wm.render(gl, shader, trans_mat * rot_mat * scale_mat)
            }
        }
    }

    pub fn render_from_model_mat(&self, gl: &gl::Gl, shader: &render_gl::Shader, model_mat: na::Matrix4::<f32>) {
        match &self.model {
            ModelType::Cube(c) => {
                c.render(gl, shader,  model_mat);
            },
            ModelType::WaveModel(wm) => {
                wm.render(gl, shader, model_mat);
            }
        }
    }
}
