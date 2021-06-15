use nalgebra as na;

#[derive(Debug, Copy, Clone)]
pub struct Physics {
    pub pos: na::Vector3<f32>,
    pub velocity: na::Vector3<f32>,
    pub max_speed: f32,
    pub rotation: na::UnitQuaternion::<f32>,
    pub facing_dir: na::Vector3<f32>,
    pub scale: f32,
    //
    pub inverse_mass: f32,
    pub anchor_id: Option<usize>,
    pub falling: bool,
}

impl Physics {
    pub fn new() -> Physics {
        Physics {
            rotation: na::UnitQuaternion::identity(),
            pos: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            velocity: na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            facing_dir:
            na::Vector3::<f32>::new(0.0, 0.0, 0.0),
            max_speed: 5.0,
            inverse_mass: 1.0,
            scale: 1.0,
            anchor_id: None,
            falling: true,
        }
    }

    pub fn apply_transform(&mut self, transform: na::Matrix4::<f32>) {

        let identity_pos = na::Vector4::new(0.0, 0.0, 0.0, 1.0);

        let up = transform * na::Vector4::new(0.0, 0.0, 1.0, 1.0);

        (transform * identity_pos).xyz();

        self.pos = (transform * identity_pos).xyz();

        let mut rot_mat = na::Matrix3::<f32>::identity();

        rot_mat[0] = transform[0];
        rot_mat[1] = transform[1];
        rot_mat[2] = transform[2];
        rot_mat[3] = transform[4];
        rot_mat[4] = transform[5];
        rot_mat[5] = transform[6];
        rot_mat[6] = transform[8];
        rot_mat[7] = transform[9];
        rot_mat[8] = transform[10];

        self.rotation = na::UnitQuaternion::from_matrix(&rot_mat);

    }


    pub fn calculate_model_mat(&self) -> na::Matrix4::<f32> {

        let scale_mat = na::Matrix4::<f32>::new(
            self.scale, 0.0, 0.0, 0.0,
            0.0, self.scale, 0.0, 0.0,
            0.0, 0.0, self.scale, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let rot_mat = self.rotation.to_homogeneous();

        let trans_mat = na::Matrix4::new_translation(&self.pos);

        let model_mat = trans_mat * rot_mat * scale_mat;

        model_mat
    }

}
