use nalgebra as na;

use crate::cube;
use crate::render_gl;


enum ModelType {

    Cube(cube::Cube),
    WaveModel(render_gl::Model),
    Skinned(render_gl::SkinnedMesh),
    Mesh(render_gl::Mesh)
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

    pub fn skinned_model(skinned: render_gl::SkinnedMesh) -> Self {
        let model = ModelType::Skinned(skinned);
        Model {
            model
        }
    }

    pub fn mesh(mesh: render_gl::Mesh) -> Self {
        let model = ModelType::Mesh(mesh);
        Model {
            model
        }
    }


    pub fn render_from_model_mat(&self, gl: &gl::Gl, shader: &render_gl::Shader, model_mat: na::Matrix4::<f32>, bones: &Vec::<na::Matrix4::<f32>>) {
        match &self.model {
            ModelType::Cube(c) => {
                c.render(gl, shader,  model_mat);
            },
            ModelType::WaveModel(wm) => {
                wm.render(gl, shader, model_mat);
            },
            ModelType::Skinned(mesh) => {
                mesh.render(gl, shader, model_mat, bones);
            },
            ModelType::Mesh(mesh) => {
                mesh.render(gl, shader, model_mat);
            }

        }
    }
}
