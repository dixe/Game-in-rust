use crate::entity::*;


pub struct Entities {
    pub player: Entity,
    pub default_weapon: Entity,
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

    pub fn count(&self) -> usize {
        self.entities.len()
    }

}

impl Entities {

    pub fn new() -> Self {
        Entities {
            enemies: EntitiesCollection::new(),
            weapons: EntitiesCollection::new(),
            player: Entity::new(None, "Placeholder".to_string()),
            default_weapon: Entity::new(None, "default_weapon".to_string())
        }
    }


    pub fn hitbox_entities(&self) -> Vec::<&Entity>{

        self.values()
    }

    pub fn values(&self) -> Vec::<&Entity>{

        let mut res = Vec::new();

        res.push(&self.player);

        // and weapons used

        match self.weapons.entities.get(&self.player.weapon_id) {
            Some(w) => {
                res.push(w);
            },
            _ =>{}
        }


        for e in self.enemies.entities.values() {
            res.push(e);
        }

        res


    }


}
