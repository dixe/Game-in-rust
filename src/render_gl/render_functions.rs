use gl;
use crate::entity;
use crate::render_gl;
use crate::game;


use nalgebra as na;

fn calculate_model_mat(physics: &entity::Physics) -> na::Matrix4::<f32> {

    let scale_mat = na::Matrix4::<f32>::new(
        physics.scale, 0.0, 0.0, 0.0,
        0.0, physics.scale, 0.0, 0.0,
        0.0, 0.0, physics.scale, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let pos = physics.pos;

    let rot_mat = na::Matrix4::<f32>::new_rotation(physics.rotation);

    let trans_mat = na::Matrix4::new_translation(&pos);

    let model_mat = trans_mat * rot_mat * scale_mat;

    model_mat
}



pub fn render_entity(entity: &entity::Entity, entities: &entity::Entities, model: &entity::Model, gl: &gl::Gl, shader: &render_gl::Shader) {

    let physics = match game::get_absoulte_physics(entity.physics.entity_id, entities) {
        Some(physics) => physics,
        _ => return,
    };

    let model_mat = calculate_model_mat(&physics);

    model.render_from_model_mat(gl, shader, model_mat, entity.animation_player.as_ref().map(|ap| &ap.bones));
}
