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

        //        let scale_mat = na::Matrix4::<f32>::identity() * na::Vector4::new(scale.x, scale.y, scale.z, 1.0).transpose()

        let scale_mat =  na::Matrix4::<f32>::identity() *
            na::Matrix4::<f32>::new(
                scale.x, 0.0, 0.0, 0.0,
                0.0,scale.y, 0.0, 0.0,
                0.0, 0.0, scale.z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            );

        self.model_mat = scale_mat * self.model_mat;
    }

    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, pos: na::Vector3::<f32>) {

        let trans = na::Matrix4::new_translation(&pos);

        self.model.render(gl, shader, trans * self.model_mat);
    }

}
