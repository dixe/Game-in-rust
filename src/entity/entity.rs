use nalgebra as na;

use crate::cube;

#[derive(Copy, Clone)]
pub struct Entity {
    pub id: usize,
    pub pos: na::Vector3::<f32>,
    pub velocity: na::Vector3::<f32>,
    pub max_speed: f32,
    pub acceleration: f32,
    //
    pub model_id: usize,

}


impl Entity {


    pub fn set_position(&mut self, pos: na::Vector3::<f32>) {
        self.pos = pos;
    }


    pub fn set_velocity(&mut self, vel: na::Vector3::<f32>) {
        self.velocity = vel;
    }

    pub fn render(&self, gl: &gl::Gl, model: &cube::Cube, projection: &na::Matrix4<f32>, view: &na::Matrix4<f32>) {

        let trans = na::Matrix4::new_translation(&self.pos);

        let mut model_mat = na::Matrix4::<f32>::identity();

        model_mat = trans * model_mat;

        model.render(gl, *projection, *view, model_mat);



    }
}
