use nalgebra as na;

#[derive(Debug, Copy, Clone)]
pub struct Physics {
    pub entity_id: usize,
    pub pos: na::Vector3<f32>,
    pub velocity: na::Vector3<f32>,
    pub max_speed: f32,
    pub rotation: na::Vector3<f32>,
    pub target_dir: na::Vector3<f32>,
    pub scale: f32,
    //
    pub inverse_mass: f32,
    pub anchor_id: Option<usize>,
}

impl Physics {
    pub fn new(entity_id: usize) -> Physics {
        Physics {
            entity_id: entity_id,
            rotation: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            pos: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            velocity: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            target_dir: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            max_speed: 5.0,
            inverse_mass: 1.0,
            scale: 1.0,
            anchor_id: None
        }
    }
}
