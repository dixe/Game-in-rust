use nalgebra as na;

use crate::entity::{Physics, Model, Health};

#[derive(ComponentSystem)]
pub struct EntityComponentSystem {

    next_id: usize,

    models: Vec<Model>,


    // Components
    #[component = "Physics"]
    physics: std::collections::HashMap<usize, Physics>,

    #[component = "Health"]
    health: std::collections::HashMap<usize, Health>,

    model_reference: std::collections::HashMap<usize, usize>,


}


impl EntityComponentSystem {

    pub fn new () -> Self {
        return EntityComponentSystem {
            next_id: 1,
            physics: std::collections::HashMap::new(),
            health: std::collections::HashMap::new(),
            models: Vec::<Model>::new(),
            model_reference: std::collections::HashMap::new(),
        }
    }

    pub fn add_entity (&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn set_model(&mut self, entity_id: usize, model_id: usize)  {
        self.model_reference.insert(entity_id, model_id);
    }


    pub fn add_model(&mut self, model: Model) -> usize {
        self.models.push(model);

        (self.models.len() - 1) as usize

    }


    pub fn render(&self, entity_id:usize, gl: &gl::Gl, projection: &na::Matrix4<f32>, view: &na::Matrix4<f32>) {
        match self.get_physics(entity_id) {
            Some(e) => match self.models.get(e.model_id as usize) {
                Some(m) => m.render(gl, projection, view, e.pos),
                None => {}
            },
            None => {}
        };

    }

}
