use gl;
use crate::entity;
use crate::render_gl;






pub fn render_entity(entity: &entity::Entity, _entities: &entity::Entities, model: &entity::Model, gl: &gl::Gl, shader: &render_gl::Shader) {

    model.render_from_model_mat(gl, shader, entity.physics.calculate_model_mat(), &entity.bones);

}
