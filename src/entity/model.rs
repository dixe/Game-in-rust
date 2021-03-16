use crate::cube;

use crate::render_gl;
use nalgebra as na;


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

    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, pos: na::Vector3::<f32>, rotation_sin: f32, rotation_cos: f32, scale: f32) {

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


        let trans_mat = na::Matrix4::new_translation(&pos);

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
