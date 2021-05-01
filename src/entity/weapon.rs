use crate::physics::CollisionBox;
use crate::entity::*;

pub fn create_weapon(entity_id: usize, model_name: String, hitboxes:  &Vec::<(String,Vec<na::Vector3::<f32>>)>) -> Entity {

    let physics = Physics::new(entity_id);

    let health = Health::new(0.0);

    let mut entity = Entity::new(physics, health, None, model_name);

    for hitbox_kv in hitboxes {
        let mut hb = CollisionBox::from_mesh_data(&hitbox_kv.1);
        hb.name = hitbox_kv.0.clone();
        entity.hit_boxes.push(hb);
    }

    entity
}
