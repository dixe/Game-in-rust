use nalgebra as na;


#[derive(Debug,Copy, Clone)]
pub struct Physics {

    pub entity_id: usize,
    pub pos: na::Vector3::<f32>,
    pub velocity: na::Vector3::<f32>,
    pub max_speed: f32,
    pub rotation_sin: f32,
    pub rotation_cos: f32,
    //
    pub model_id: usize,
    pub inverse_mass: f32,
}




impl Physics {

    pub fn new(entity_id: usize, model_id: usize) -> Physics {
        Physics {
            entity_id: entity_id,
            rotation_sin: 0.0,
            rotation_cos: 1.0,
            pos: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            velocity: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            max_speed: 10.0,
            //TODO remove from phyiscs
            model_id: model_id,
            inverse_mass: 1.0,
        }
    }
}
