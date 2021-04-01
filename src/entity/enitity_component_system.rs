
use crate::entity::*;



#[derive(ComponentSystem)]
pub struct EntityComponentSystem {

    next_id: usize,
    models: Vec<Model>,

    // Components
    #[component = "Physics"]
    pub physics: std::collections::HashMap<usize, Physics>,
    #[component = "Health"]
    health: std::collections::HashMap<usize, Health>,
    #[component = "Shooter"]
    pub shooter: std::collections::HashMap<usize, Shooter>,
    #[component = "Shot"]
    pub shot: std::collections::HashMap<usize, Shot>,
    #[component = "ActionsInfo"]
    pub actions_info: std::collections::HashMap<usize, ActionsInfo>,
    #[component = "AnchorPoint"]
    pub anchor_point: std::collections::HashMap<usize, AnchorPoint>,

    model_reference: std::collections::HashMap<usize, usize>,


}


impl EntityComponentSystem {

    pub fn new () -> Self {
        return EntityComponentSystem {
            next_id: 1,
            physics: std::collections::HashMap::new(),
            health: std::collections::HashMap::new(),
            shooter: std::collections::HashMap::new(),
            shot: std::collections::HashMap::new(),
            actions_info: std::collections::HashMap::new(),
            models: Vec::<Model>::new(),
            model_reference: std::collections::HashMap::new(),
            anchor_point: std::collections::HashMap::new(),
        }
    }

    pub fn add_entity (&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn get_model(&self, entity_id: usize) -> Option<&Model> {
        self.model_reference.get(&entity_id).and_then(|index| self.models.get(*index))
    }

    pub fn set_model(&mut self, entity_id: usize, model_id: usize)  {
        self.model_reference.insert(entity_id, model_id);
    }


    pub fn add_model(&mut self, model: Model) -> usize {
        self.models.push(model);

        (self.models.len() - 1) as usize

    }
}
