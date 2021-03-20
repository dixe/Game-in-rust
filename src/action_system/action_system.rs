use crate::entity;


pub fn update_actions(actions: &mut std::collections::HashMap<usize, entity::AnimationsInfo>, physics: &mut std::collections::HashMap<usize, entity::Physics>, delta: f32) {


    for action in actions.values_mut() {
        match physics.get_mut(&action.entity_id) {
            Some(physics) => {
                action.update(physics, delta);
            },
            _ => {}
        };
    }
}
