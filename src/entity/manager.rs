use nalgebra as na;

use crate::cube;

use crate::entity::Entity;


pub struct EntityManager {

    next_id: usize,
    pub entities: std::collections::HashMap<usize, Entity>,
    models: Vec<cube::Cube>,

}


impl EntityManager {

    pub fn new () -> Self {
        return EntityManager {
            next_id: 1,
            entities: std::collections::HashMap::new(),
            models: Vec::<cube::Cube>::new()
        }
    }


    fn next_entity (&mut self, model_id: usize, pos: na::Vector3::<f32>, direction: na::Vector3::<f32>) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let e = Entity {
            id,
            model_id,
            pos,
            velocity: direction,
            acceleration: 10.0,
            max_speed: 10.0
        };

        self.entities.insert(id, e);
        id
    }


    pub fn update_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, entity);
    }


    pub fn remove_entity(&mut self, id: usize) {
        self.entities.remove(&id);
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


    pub fn get_entity(&self, id: usize) -> Option<Entity> {
        match &self.entities.get(&id) {
            Some(e) => Some(**e),
            None => None
        }
    }


    pub fn add_model(&mut self, model: cube::Cube) -> usize {
        self.models.push(model);

        (self.models.len() - 1) as usize


    }


    pub fn render(&self, entity_id:usize, gl: &gl::Gl, projection: &na::Matrix4<f32>, view: &na::Matrix4<f32>) {
        match self.get_entity(entity_id) {
            Some(e) => match self.models.get(e.model_id as usize) {
                Some(m) => e.render(gl, m, projection, view),
                None => {}
            },
            None => {}
        };

    }


    /*

    pub fn set_player (&mut self, model_id: usize, pos: na::Vector3::<f32>) -> usize {

    let id = self.next_entity(model_id, pos, empty_vec());
    self.player_id = id;
    id
}

}


    pub fn player(&self) -> Option<&Entity> {
    self.get_entity(self.player_id)
}


    pub fn player_mut(&mut self) -> Option<&mut Entity> {
    self.get_entity_mut(self.player_id)
}

    pub fn enemiey_mut(&mut self, enemy_id: usize) -> Option<&mut Entity> {
    self.get_entity_mut(enemy_id)
}
     */


}


fn empty_vec() -> na::Vector3::<f32> {
    na::Vector3::new(0.0, 0.0, 0.0)
}
