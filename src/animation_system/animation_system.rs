use crate::game;
use crate::entity;


pub fn update_animations(animations: &mut std::collections::HashMap<usize, entity::AnimationData>, physics: &mut std::collections::HashMap<usize, entity::Physics>, delta: i32) {

    for animation in animations.values_mut() {
        match physics.get_mut(&animation.entity_id) {
            Some(physics) => {
                animation.update(physics, delta);
            },
            _ => {}
        };
    }
}
