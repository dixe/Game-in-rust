use gl;
use crate::entity;
use crate::render_gl;
use crate::game;


use nalgebra as na;


pub fn render_entity(entity: &entity::Entity, entities: &entity::Entities, model: &entity::Model, gl: &gl::Gl, shader: &render_gl::Shader) {

    model.render_from_model_mat(gl, shader, entity.physics.calculate_model_mat(), &entity.bones);

}
