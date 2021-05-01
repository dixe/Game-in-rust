use crate::entity::*;


pub struct Entities {
    pub player: Entity,
    pub weapons: EntitiesCollection,
    pub enemies: EntitiesCollection,
}

pub struct EntitiesCollection {
    next_id: usize,
    pub entities: std::collections::HashMap::<usize, Entity>,
}

impl EntitiesCollection {
    pub fn new() -> Self {
        EntitiesCollection {
            next_id: 0,
            entities: std::collections::HashMap::<usize, Entity>::new(),
        }
    }

    pub fn add(&mut self, entity: entity::Entity) -> usize {
        let id = self.next_id;
        self.entities.insert(id, entity);
        id
    }

    pub fn get(&self, id: usize) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }


    pub fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, usize, Entity> {
        self.entities.values_mut()
    }


    pub fn values(&self) -> std::collections::hash_map::Values<'_, usize, Entity> {
        self.entities.values()
    }

}

impl Entities {

    pub fn new() -> Self {
        Entities {
            enemies: EntitiesCollection::new(),
            weapons: EntitiesCollection::new(),
            player: Entity::new(None, "Placeholder".to_string())
        }
    }


    pub fn hitbox_entities(&self) -> Vec::<&Entity>{

        self.values()
    }

    pub fn values(&self) -> Vec::<&Entity>{

        let mut res = Vec::new();

        res.push(&self.player);

        for w in self.weapons.entities.values() {
            res.push(&w);
        }

        for e in self.enemies.entities.values() {
            res.push(&e);
        }


        res

    }

    pub fn values_mut(&mut self) -> Vec::<&mut Entity>{


        let mut res = Vec::new();

        res.push(&mut self.player);

        for w in self.weapons.entities.values_mut() {
            res.push(w);
        }

        for e in &mut self.enemies.entities.values_mut() {
            res.push(e);
        }


        res

    }



    /*

    pub fn hammer(&self) -> &Entity {
    match self.entities_map.get(&self.hammer_id) {
    Some(e) => e,
    None => panic!("No hammer set")
}
}

    pub fn hammer_mut(&mut self) -> &mut Entity {
    match self.entities_map.get_mut(&self.hammer_id) {
    Some(e) => e,
    None => panic!("No hammer set")
}
}

    pub fn dummy(&self) -> &Entity {
    match self.entities_map.get(&self.dummy_id) {
    Some(e) => e,
    None => panic!("No hammer set")
}
}



    pub fn get(&self, entity_id: usize) -> Option<&Entity> {
    self.entities_map.get(&entity_id)
}

    pub fn get_mut(&mut self, entity_id: usize) -> Option<&mut Entity> {
    self.entities_map.get_mut(&entity_id)
}

    pub fn add(&mut self, entity: Entity) -> usize {

    let id = self.next_id;
    self.next_id += 1;

    self.entities_map.insert(id, entity);
    id
}

    pub fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, usize, Entity> {
    self.entities_map.values_mut()
}


    pub fn values(&self) -> std::collections::hash_map::Values<'_, usize, Entity> {
    self.entities_map.values()
}

    pub fn set_physics(&mut self, entity_id: usize, physics: Physics) {

    match self.entities_map.get_mut(&entity_id) {
    Some(e) => {
    e.physics = physics  }
    _ => {}
};
}
     */
}
