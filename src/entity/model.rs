use crate::cube;

use crate::render_gl;
use nalgebra as na;


pub struct Model {
    model: cube::Cube,
    model_mat: na::Matrix4::<f32>
}


impl Model {

    pub fn new(model: cube::Cube) -> Self {
        Model {
            model,
            model_mat: na::Matrix4::identity()
        }
    }


    pub fn scale(&mut self, scale: &na::Vector3<f32>) {

        let scale_mat =
            na::Matrix4::<f32>::new(
                scale.x, 0.0, 0.0, 0.0,
                0.0,scale.y, 0.0, 0.0,
                0.0, 0.0, scale.z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            );

        self.model_mat = scale_mat * self.model_mat;
    }

    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, pos: na::Vector3::<f32>, rotation_sin: f32, rotation_cos: f32) {

        let rot = na::Matrix4::<f32>::new(
            rotation_cos, -rotation_sin, 0.0, 0.0,
            rotation_sin, rotation_cos, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );


        let trans = na::Matrix4::new_translation(&pos);
        self.model.render(gl, shader, trans * rot * self.model_mat);
    }

}
