use nalgebra as na;

use crate::entity::{Physics, Model, Health};

#[derive(ComponentSystem)]
pub struct EntityComponentSystem {

    next_id: usize,
    #[component = "Physics"]
    pub physics: std::collections::HashMap<usize, Physics>,
    models: Vec<Model>,
    pub healths: std::collections::HashMap<usize, Health>,
    pub model_reference: std::collections::HashMap<usize, usize>,


}


impl EntityComponentSystem {

    pub fn new () -> Self {
        return EntityComponentSystem {
            next_id: 1,
            physics: std::collections::HashMap::new(),
            healths: std::collections::HashMap::new(),
            models: Vec::<Model>::new(),
            model_reference: std::collections::HashMap::new(),
        }
    }



    pub fn set_health(&mut self, entity_id: usize, hp: Health) {
        self.healths.insert(entity_id, hp);
    }




    pub fn add_entity (&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }


    pub fn get_physics(&self, id: usize) -> Option<Physics> {
        match &self.physics.get(&id) {
            Some(e) => Some(**e),
            None => None
        }
    }


    pub fn get_health(&self, id: usize) -> Option<Health> {
        match &self.healths.get(&id) {
            Some(e) => Some(**e),
            None => None
        }
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


fn empty_vec() -> na::Vector3::<f32> {
    na::Vector3::new(0.0, 0.0, 0.0)
}
