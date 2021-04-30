use gl;
use crate::entity;
use crate::render_gl;
use crate::game;


use nalgebra as na;


pub fn render_entity(entity: &entity::Entity, entities: &entity::Entities, model: &entity::Model, gl: &gl::Gl, shader: &render_gl::Shader) {

    let physics = match game::get_absoulte_physics(entity.physics.entity_id, entities) {
        Some(physics) => physics,
        _ => return,
    };

    let model_mat = physics.calculate_model_mat();

    model.render_from_model_mat(gl, shader, model_mat, &entity.bones);
}
