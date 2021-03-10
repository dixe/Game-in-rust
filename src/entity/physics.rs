use nalgebra as na;


#[derive(Copy, Clone)]

pub struct Physics {

    pub pos: na::Vector3::<f32>,
    pub velocity: na::Vector3::<f32>,
    pub max_speed: f32,
    pub acceleration: f32,
    //
    pub model_id: usize,

}


impl Physics {

    pub fn set_position(&mut self, pos: na::Vector3::<f32>) {
        self.pos = pos;
    }


    pub fn set_velocity(&mut self, vel: na::Vector3::<f32>) {
        self.velocity = vel;
    }
}
