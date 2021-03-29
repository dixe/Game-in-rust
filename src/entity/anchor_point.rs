use nalgebra as na;


#[derive(Debug, Copy, Clone)]
pub struct AnchorPoint {
    pub pos: na::Vector3::<f32>,
    pub normal: na::Vector3::<f32>,
}


impl AnchorPoint {
    pub fn default() -> AnchorPoint {
        AnchorPoint {
            pos: na::Vector3::new(0.0, 0.0, 0.0),
            normal: na::Vector3::new(0.0, 0.0, 1.0),
        }
    }
}
