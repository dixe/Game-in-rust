use nalgebra as na;

use crate::entity::{Physics, Model, Health};


pub struct EntityComponentSystem {

    next_id: usize,
    pub entities: std::collections::HashMap<usize, Physics>,
    models: Vec<Model>,
    pub healths: std::collections::HashMap<usize, Health>,

}


impl EntityComponentSystem {

    pub fn new () -> Self {
        return EntityComponentSystem {
            next_id: 1,
            entities: std::collections::HashMap::new(),
            healths: std::collections::HashMap::new(),
            models: Vec::<Model>::new()
        }
    }


    fn next_entity (&mut self, model_id: usize, pos: na::Vector3::<f32>, direction: na::Vector3::<f32>) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let e = Physics {
            model_id,
            pos,
            velocity: direction,
            acceleration: 10.0,
            max_speed: 10.0
        };

        self.entities.insert(id, e);
        id
    }


    pub fn set_health(&mut self, entity_id: usize, hp: Health) {
        self.healths.insert(entity_id, hp);
    }


    pub fn update_entity(&mut self, entity_id: usize, entity: Physics) {
        self.entities.insert(entity_id, entity);
    }


    pub fn remove_entity(&mut self, id: usize) {

        self.entities.remove(&id);
        self.healths.remove(&id);
    }



    pub fn add_entity (&mut self, model_id: usize, pos: na::Vector3::<f32>) -> usize {
        // maybe check it we have dead ones?
        let id = self.next_entity(model_id, pos, empty_vec());
        id
    }

    pub fn add_entity_with_vel(&mut self, model_id: usize, pos: na::Vector3::<f32>, vel: na::Vector3::<f32>) -> usize {
        // maybe check it we have dead ones?
        let id = self.next_entity(model_id, pos, vel);
        id
    }


    pub fn get_physics(&self, id: usize) -> Option<Physics> {
        match &self.entities.get(&id) {
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
