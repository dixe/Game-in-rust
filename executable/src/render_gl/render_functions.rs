use gl;
use crate::entity;
use crate::render_gl;


pub fn render_entity(entity: &entity::Entity, model: &entity::Model, gl: &gl::Gl, shader: &render_gl::Shader) {
    model.render_from_model_mat(gl, shader, entity.base_entity.physics.calculate_model_mat(), &entity.bones);
}




pub fn render_world( model: &entity::Model, gl: &gl::Gl, shader: &render_gl::Shader) {
    model.render_from_model_mat(gl, shader, na::Matrix4::identity(), &Vec::new());
}
