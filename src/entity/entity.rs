use nalgebra as na;

use crate ::cube;


pub struct Entity {

    pub pos: na::Vector3::<f32>,
    pub velocity: na::Vector3::<f32>,
    pub max_speed: f32,
    pub acceleration: f32,
    model: cube::Cube,

}


impl Entity {

    pub fn new (cube: cube::Cube, pos: na::Vector3::<f32>) -> Self {
        Entity {
            model: cube,
            pos,
            velocity: na::Vector3::<f32>::new(0.0,0.0,0.0),
            acceleration: 0.05,
            max_speed: 0.15
        }
    }


    pub fn set_position(&mut self, pos: na::Vector3::<f32>) {
        self.pos = pos;
    }


    pub fn set_velocity(&mut self, vel: na::Vector3::<f32>) {
        self.velocity = vel;
    }


    pub fn render(&self, gl: &gl::Gl, projection: na::Matrix4<f32>, view: na::Matrix4<f32>) {

        let trans = na::Matrix4::new_translation(&self.pos);

        let mut model_mat = na::Matrix4::<f32>::identity();

        model_mat = trans * model_mat;

        self.model.render(gl, projection, view, model_mat);

    }
}
