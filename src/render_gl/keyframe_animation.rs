use crate::render_gl::{Mesh, Skeleton};


#[derive(Debug)]
pub struct KeyframeAnimation {
    name: String,
    frame_rate: i64,
    duration: f32,
    skeleton: Skeleton,
    key_frame: KeyFrame,
    model: Mesh,
}





#[derive(Debug)]
pub struct KeyFrame {
    name: String,
    joint_transformation: Vec<Transformation>,
}



#[derive(Debug)]
pub struct Transformation {
    translation: na::Vector3::<f32>,
    orientation: na::UnitQuaternion::<f32>,
    scale: na::Vector3::<f32>
}
