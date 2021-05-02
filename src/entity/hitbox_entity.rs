use crate::physics::CollisionBox;
use crate::entity::*;

pub fn create_hitbox_entity(model_name: String, hitboxes:  &Vec::<(String,Vec<na::Vector3::<f32>>)>) -> Entity {

    let mut entity = Entity::new(None, model_name);

    for hitbox_kv in hitboxes {
        let mut hb = CollisionBox::from_mesh_data(&hitbox_kv.1);
        hb.name = hitbox_kv.0.clone();
        entity.hit_boxes.push(hb);
    }

    entity
}
