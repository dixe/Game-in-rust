#[derive(Debug)]
pub struct Skeleton {
    joints: Vec<Joint>
}

#[derive(Debug)]
pub struct Joint {
    name: String,
    parent_index: usize,
    inverse_bind_pose: na::Matrix4::<f32>
}
