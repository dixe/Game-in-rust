use crate::physics::CollisionBox;
use crate::entity::*;

pub fn add_hitbox_to_entity(entity: &mut Entity,  hitboxes:  &Vec::<(String,Vec<na::Vector3::<f32>>)>) {

    for hitbox_kv in hitboxes {
        let mut hb = CollisionBox::from_mesh_data(&hitbox_kv.1);
        hb.name = hitbox_kv.0.clone();
        entity.hit_boxes.push(hb);
    }
}
