use crate::render_gl::{Mesh, Skeleton, Joint};


#[derive(Debug)]
pub struct KeyframeAnimation {
    pub name: String,
    pub duration: f32,
    pub skeleton: Skeleton,
    pub key_frames: Vec<KeyFrame>,
}

#[derive(Debug)]
pub struct KeyFrame {
    pub joints: Vec<Transformation>,
}

#[derive(Debug, Copy, Clone)]
pub struct Transformation {
    pub translation: na::Vector3::<f32>,
    pub rotation: na::UnitQuaternion::<f32>,
}

impl Transformation {

    pub fn identity(joint: &Joint) -> Self {
        Transformation {
            translation: joint.translation,
            rotation: joint.rotation,
        }
    }

    pub fn rotation_euler(joint: &Joint, roll: f32, pitch: f32, yaw: f32) -> Self {
        Transformation {
            translation: joint.translation,
            rotation: na::UnitQuaternion::from_euler_angles(roll, pitch, yaw),
        }
    }
}

impl KeyframeAnimation {

    pub fn new(name: &str, duration: f32, skeleton: Skeleton, key_frames: Vec<KeyFrame>) -> KeyframeAnimation {

        KeyframeAnimation {
            name: name.to_string(),
            duration,
            skeleton,
            key_frames,
        }
    }

    pub fn move_to_key_frame(&mut self, bones: &mut [na::Matrix4::<f32>], keyframe: usize, t: f32) {

        // interpolate joints new transformation


        for i in 0..self.skeleton.joints.len() {


            let current_transformation = match keyframe {
                0 => {
                    self.skeleton.joints[i].transformation()
                },
                n => {
                    self.key_frames[n - 1].joints[i]
                }
            };


            let target_joint = &self.key_frames[keyframe].joints[i];


            let rotation = current_transformation.rotation.slerp(&target_joint.rotation, t);
            let translation = current_transformation.translation * (1.0-t) + target_joint.translation * t;

            let local_matrix = self.skeleton.joints[i].get_local_matrix_data(rotation, translation);

            let mut world_matrix = local_matrix;

            let parent_index = self.skeleton.joints[i].parent_index;
            if parent_index  != 255 {
                world_matrix = self.skeleton.joints[parent_index].world_matrix * local_matrix;
            }

            self.skeleton.joints[i].world_matrix = world_matrix;
            bones[i] = world_matrix * self.skeleton.joints[i].inverse_bind_pose;

        }
    }
}
