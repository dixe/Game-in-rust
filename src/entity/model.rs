use crate::cube;

use crate::render_gl;
use nalgebra as na;


pub struct Model {
    model: cube::Cube,

}


impl Model {

    pub fn new(model: cube::Cube) -> Self {
        Model {
            model,

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
        self.model.render(gl, shader, trans_mat * rot_mat * scale_mat);
    }

}
