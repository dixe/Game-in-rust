use nalgebra as na;

use crate::render_gl::{Mesh, Skeleton, Joint};
use crate::resources::Resources;

#[derive(Debug)]
pub struct KeyframeAnimation {
    pub name: String,
    pub duration: f32,
    pub skeleton: Skeleton,
    pub key_frames: Vec<KeyFrame>,
}

#[derive(Debug, Clone)]
pub struct KeyFrame {
    pub joints: Vec<Transformation>,
}

#[derive(Debug, Copy, Clone)]
pub struct Transformation {
    pub translation: na::Vector3::<f32>,
    pub rotation: na::UnitQuaternion::<f32>,
}


fn load_joint_data(bvh: &bvh_anim::Bvh, joint: &bvh_anim::JointData, frame: usize) -> Transformation {

    let channels = joint.channels();


    let offset = joint.offset();

    let mut x = offset.x;
    let mut y = offset.y;
    let mut z = offset.z;

    // should be the bones default
    // which might not be 0
    let mut rx = 0.0;
    let mut ry = 0.0;
    let mut rz = 0.0;

    for c in joint.channels() {
        match c.channel_type() {
            bvh_anim::ChannelType::RotationX => {
                rx = bvh.get_motion(frame, c);
            },
            bvh_anim::ChannelType::RotationY => {
                ry = bvh.get_motion(frame, c);
            },
            bvh_anim::ChannelType::RotationZ => {
                rz = bvh.get_motion(frame, c);
            },

            bvh_anim::ChannelType::PositionX => {
                x = bvh.get_motion(frame, c);
            },
            bvh_anim::ChannelType::PositionY => {
                y = bvh.get_motion(frame, c);
            },
            bvh_anim::ChannelType::PositionZ => {
                z = bvh.get_motion(frame, c);
            },
        };
    }



    let translation = na::Vector3::new(x,y,z);

    let rotation = na::UnitQuaternion::from_euler_angles(rx.to_radians(), ry.to_radians(), rz.to_radians());


    Transformation {
        translation,
        rotation
    }
}




pub fn key_frames_from_bvh(res: &Resources, joint_map: &std::collections::HashMap::<std::string::String, usize>) -> Result<Vec<KeyFrame>, failure::Error> {
    // some kind of joint name to index in result mapping

    let bvh = res.load_bvh("animations/test/knees.bvh")?;
    //let bvh = res.load_bvh("animations/run/run.bvh")?;
    let mut res = Vec::new();

    //TODO get frames from file??

    for frame in 0..bvh.frames().len() {

        let mut transforms = Vec::new();

        let mut i = 0;
        for j in bvh.joints() {

            let data = j.data();
            let name: String = data.name().to_string();

            let index = match joint_map.get(&name) {
                Some(i) => *i,
                None => {continue}
            };

            let transform = load_joint_data(&bvh, &data, frame);
            println!("joint: {} transform\n {:#?}", name, transform);
            println!("Keyframe index {} {:#?}", i, name);
            transforms.push(transform);
            i += 1;
        }
        println!(" keyframe joints {:#?}", transforms.len());
        res.push( KeyFrame {
            joints: transforms
        });

    }

    Ok(res)
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


        let mut world_matrices = Vec::new();
        for i in 0..self.skeleton.joints.len() {


            let current_transformation = match keyframe {
                0 => {

                    /*
                    println!("joint {:#?}", self.skeleton.joints[i].name);
                    println!("Skel transform {:#?}", self.skeleton.joints[i].transformation());

                    println!("target {:#?}", &self.key_frames[keyframe].joints[i]);
                     */
                    self.key_frames[self.key_frames.len() - 1].joints[i]

                },
                n => {
                    self.key_frames[n - 1].joints[i]
                }
            };


            let target_joint = &self.key_frames[keyframe].joints[i];


            let rotation = current_transformation.rotation.slerp(&target_joint.rotation, t);
            let translation = current_transformation.translation * (1.0 - t) + target_joint.translation * t;

            /*
            let rotation = current_transformation.rotation;
            let translation = current_transformation.translation;

            let translation = self.skeleton.joints[i].translation;
            let rotation = self.skeleton.joints[i].rotation;
             */
            let local_matrix  = self.skeleton.joints[i].get_local_matrix_data(rotation, translation);

            let mut world_matrix = local_matrix;

            let parent_index = self.skeleton.joints[i].parent_index;
            if parent_index  != 255 {
                world_matrix = world_matrices[parent_index] * local_matrix;
            }

            world_matrices.push(world_matrix);

            //self.skeleton.joints[i].world_matrix = world_matrix;
            bones[i] = world_matrix * self.skeleton.joints[i].inverse_bind_pose;

        }
    }
}
