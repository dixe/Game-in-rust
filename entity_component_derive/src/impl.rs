impl EntityComponentSystem {
    pub fn set_physics(&mut self, entity_id: usize, component: Physics) {
        self.physics.insert(entity_id, component);
    }

    pub fn remove_entity ( & mut self , id : usize ) {
        pub fn remove_entity(&mut self, id: usize) {
            self.physics.remove(&id)
        }
    }
}
