use crate::entity;

use nalgebra as na;

pub fn calculate_model_mat(physics: &entity::Physics, anchor_physics: Option<&entity::Physics>) -> na::Matrix4::<f32> {

    let scale_mat = na::Matrix4::<f32>::new(
        physics.scale, 0.0, 0.0, 0.0,
        0.0, physics.scale, 0.0, 0.0,
        0.0, 0.0, physics.scale, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let mut pos = physics.pos;

    let rot_mat = na::Matrix4::<f32>::new_rotation(physics.rotation);

    let trans_mat = na::Matrix4::new_translation(&pos);

    let mut model_mat = trans_mat * rot_mat * scale_mat;

    match anchor_physics {
        Some(anchor) => {
            let anchor_trans = na::Matrix4::new_translation(&anchor.pos) ;
            let anchor_rot = na::Matrix4::<f32>::new_rotation(anchor.rotation);
            model_mat = anchor_trans * anchor_rot * model_mat;

        },
        _ => {}
    }
    model_mat
}
