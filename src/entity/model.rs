use nalgebra as na;

use crate::cube;
use crate::render_gl;


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
