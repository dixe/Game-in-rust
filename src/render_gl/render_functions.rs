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

    let mut model_mat = trans_mat * rot_mat * scale_mat;

    model_mat
}



pub fn render(ecs: &entity::EntityComponentSystem, entity_id: usize, gl: &gl::Gl, shader: &render_gl::Shader) {

    let physics = match game::get_absoulte_physics(entity_id, ecs) {
        Some(physics) => physics,
        _ => return,
    };


    match ecs.models.get(physics.model_id) {
        Some(m) => {
            let model_mat = calculate_model_mat(&physics);
            m.render_from_model_mat(gl, shader, model_mat);
        },
        _ => {}
    };
}
