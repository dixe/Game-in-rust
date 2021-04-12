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

#[derive(Debug)]
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

    let mut rx = 0.0;
    let mut ry = 0.0;
    let mut rz = 0.0;



    for c in joint.channels() {
        match c.channel_type() {
            bvh_anim::ChannelType::RotationX => {
                rx = bvh.get_motion(frame, c);
            },
            bvh_anim::ChannelType::RotationY => {
                rz = bvh.get_motion(frame, c);
            },
            bvh_anim::ChannelType::RotationZ => {
                ry = bvh.get_motion(frame, c);
            },

            // TODO should these be + instead of overwrite?
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

        //println!("{:#?}", c)

    }



    let mut translation = na::Vector3::new(x,y,z);
    let rotation = na::UnitQuaternion::from_euler_angles(rx.to_radians(), ry.to_radians(), rz.to_radians());


    Transformation {
        translation,
        rotation
    }
}


pub fn key_frames_from_bvh(res: &Resources, joint_map: &std::collections::HashMap::<std::string::String, usize>) -> Result<Vec<KeyFrame>, failure::Error> {
    // some kind of joint name to index in result mapping

    let bvh = res.load_bvh("animations/walk/walk.bvh")?;
    let mut res = Vec::new();
    for frame in &[0, 19, 39, 59] {


        let mut transforms = Vec::new();
        println!("FRAME : {:#?}", frame);
        for j in bvh.joints() {
            let data = j.data();
            let name: String = data.name().to_string();

            let index = match joint_map.get(&name) {
                Some(i) => *i,
                None => {continue}
            };
            let transform = load_joint_data(&bvh, &data, *frame);
            //println!("joint: {} transform\n {:#?}", name, transform);
            transforms.push(transform);
        }

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


        for i in 0..self.skeleton.joints.len() {


            let current_transformation = match keyframe {
                0 => {

                    /*
                    println!("joint {:#?}", self.skeleton.joints[i].name);
                    println!("Skel transform {:#?}", self.skeleton.joints[i].transformation());

                    println!("target {:#?}", &self.key_frames[keyframe].joints[i]);
                     */
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
