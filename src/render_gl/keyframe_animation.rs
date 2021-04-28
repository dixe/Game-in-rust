use nalgebra as na;
use std::collections::{HashMap};

use crate::render_gl::{Skeleton, Joint};

use crate::resources;


#[derive(Debug, Clone)]
pub struct KeyframeAnimation {
    pub duration: f32,
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



#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "No root found")]
    NoRootFound,
    #[fail(display = "Resource Error")]
    ResourceError(resources::Error),
    #[fail(display = "gltf Error")]
    GltfError(gltf::Error),
}



impl From<resources::Error> for Error {
    fn from(other: resources::Error) -> Self {
        Error::ResourceError(other)
    }
}


impl From<gltf::Error> for Error {
    fn from(other: gltf::Error) -> Self {
        Error::GltfError(other)
    }
}

#[derive(Clone)]
pub struct PlayerAnimations {
    pub walk: KeyframeAnimation,
    pub t_pose: KeyframeAnimation,
    pub idle: KeyframeAnimation,
}



pub fn load_animations(file_path: &str, skeleton: &Skeleton) -> Result<PlayerAnimations, Error> {

    let animations = key_frames_from_gltf(file_path, skeleton)?;

    let t_pose_frames = animations.get("t_pose").unwrap();

    let walk_frames = animations.get("walk").unwrap();

    let idle_frames = animations.get("idle").unwrap();

    let t_pose = KeyframeAnimation::new(1.0, t_pose_frames.clone());

    let walk = KeyframeAnimation::new(0.7, walk_frames.clone());

    let idle = KeyframeAnimation::new(2.0, idle_frames.clone());


    Ok(PlayerAnimations {
        t_pose,
        walk,
        idle,
    })
}


fn key_frames_from_gltf(file_path: &str, skeleton: &Skeleton) -> Result<HashMap<String,Vec<KeyFrame>>, Error> {
    // should be in resources, but atm the file is not in resources
    let (gltf, buffers, _) = gltf::import(file_path)?;


    let mut joints_indexes: std::collections::HashMap::<String, usize> = std::collections::HashMap::new();

    for i in 0..skeleton.joints.len() {
        joints_indexes.insert(skeleton.joints[i].name.clone(), i);
    }

    let mut res = HashMap::<String, Vec<KeyFrame>>::new();

    for ani in gltf.animations() {

        let name = match ani.name() {
            Some(n) => n.to_string(),
            _ => continue
        };

        let mut frames = Vec::new();
        let mut max_frame_count = 0;

        for channel in ani.channels() {
            let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
            let mut frame_count = 0;

            for read_outputs in reader.read_outputs() {
                match read_outputs {
                    gltf::animation::util::ReadOutputs::Translations(ts) => {
                        frame_count = ts.len();
                    },
                    _=> {}


                }
            }

            max_frame_count = usize::max(max_frame_count, frame_count);
        }


        // fill frames with

        for _ in 0..max_frame_count {

            frames.push(KeyFrame {
                joints: skeleton.joints.iter().map(|joint| {
                    Transformation {
                        translation: joint.translation,
                        rotation: joint.rotation
                    }
                }).collect()
            });
        }

        for channel in ani.channels() {
            let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
            let target = channel.target();

            let joints_index = match joints_indexes.get(target.node().name().unwrap()) {
                Some(i) => *i,
                _ => {
                    //println!("Skipping joint {:#?}", target.node().name().unwrap());
                    continue;
                }
            };

            let mut base_rot = na::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0);
            if target.node().name().unwrap() == "hip" {
                base_rot = na::UnitQuaternion::from_euler_angles(0.0, -90.0_f32.to_radians(), 0.0);
            }

            for read_outputs in reader.read_outputs() {
                match read_outputs {
                    gltf::animation::util::ReadOutputs::Translations(ts) => {
                        let mut i = 0;
                        for t in ts {
                            frames[i].joints[joints_index].translation = na::Vector3::new(t[0], t[1], t[2]);
                            i += 1;
                        }
                    },
                    gltf::animation::util::ReadOutputs::Rotations(rs) => {
                        let mut i = 0;
                        for r in rs.into_f32() {

                            let q = na::Quaternion::from(na::Vector4::new(r[0], r[1], r[2], r[3]));

                            frames[i].joints[joints_index].rotation = na::UnitQuaternion::from_quaternion(q) * base_rot;
                            i += 1 ;
                        }
                    },
                    gltf::animation::util::ReadOutputs::Scales(ss) => {
                        for s in ss {
                            let diff = f32::abs(3.0 - (s[0] + s[1] + s[2]));
                            if diff > 0.01 {
                                panic!("Scale was more that 0.01 it might be important\n scale was {}", diff)
                            }
                        }
                    },
                    gltf::animation::util::ReadOutputs::MorphTargetWeights(mtws) => {


                        println!("{:#?}", mtws);
                    }


                }
            }
        }

        res.insert(name, frames);
    }

    println!("Animations loaded:\n{:#?}", res.keys());



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

    pub fn empty() -> KeyframeAnimation {
        KeyframeAnimation {
            duration: 1.0,
            key_frames: Vec::new(),
        }
    }

    pub fn new(duration: f32,  key_frames: Vec<KeyFrame>) -> KeyframeAnimation {
        KeyframeAnimation {
            duration,
            key_frames,
        }
    }

    pub fn keyframe_from_t(&self, skeleton: &Skeleton, next_keyframe: usize, t: f32) -> KeyFrame {

        let mut joints = Vec::new();

        for i in 0..skeleton.joints.len() {

            let current_transformation = match next_keyframe {
                0 => {
                    self.key_frames[0].joints[i]

                },
                n => {
                    self.key_frames[n - 1].joints[i]
                }
            };

            let target_joint = &self.key_frames[next_keyframe].joints[i];

            let rotation = current_transformation.rotation.slerp(&target_joint.rotation, t);

            let translation = current_transformation.translation * (1.0 - t) + target_joint.translation * t;


            joints.push(Transformation {
                translation,
                rotation
            });
        }

        KeyFrame {
            joints,
        }
    }


    pub fn move_to_key_frame(&mut self, bones: &mut [na::Matrix4::<f32>], skeleton: &mut Skeleton, next_keyframe: usize, t: f32) {

        // interpolate joints new transformation

        let mut world_matrices = Vec::new();

        //println!("Frame {:#?}", keyframe);

        for i in 0..skeleton.joints.len() {

            let current_transformation = match next_keyframe {
                0 => {
                    self.key_frames[0].joints[i]

                },
                n => {
                    self.key_frames[n - 1].joints[i]
                }
            };

            let target_joint = &self.key_frames[next_keyframe].joints[i];

            let rotation = current_transformation.rotation.slerp(&target_joint.rotation, t);

            let translation = current_transformation.translation * (1.0 - t) + target_joint.translation * t;

            let local_matrix  = skeleton.joints[i].get_local_matrix_data(rotation, translation);

            let mut world_matrix = local_matrix;

            let parent_index = skeleton.joints[i].parent_index;
            if parent_index  != 255 {
                world_matrix = world_matrices[parent_index] * local_matrix;
            }

            world_matrices.push(world_matrix);

            skeleton.joints[i].world_matrix = world_matrix;
            bones[i] = world_matrix * skeleton.joints[i].inverse_bind_pose;

        }
    }
}
