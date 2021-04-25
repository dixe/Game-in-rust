use nalgebra as na;
use std::collections::{HashMap};

use crate::render_gl::{Skeleton, Joint};

use crate::resources;


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

pub struct PlayerAnimations {
    pub run: KeyframeAnimation,
    pub t_pose: KeyframeAnimation,
}



pub fn load_player_animations(skeleton: &Skeleton) -> Result<PlayerAnimations, Error> {

    let animations = key_frames_from_gltf(skeleton)?;


    let t_pose_frames = animations.get("t_pose").unwrap();

    let run_frames = animations.get("run").unwrap();

    let t_pose = KeyframeAnimation::new("t_pose", 1.0, skeleton.clone(), t_pose_frames.clone());

    let run = KeyframeAnimation::new("run", 1.0, skeleton.clone(), run_frames.clone());


    Ok(PlayerAnimations {
        t_pose,
        run
    })
}


fn key_frames_from_gltf(skeleton: &Skeleton) -> Result<HashMap<String,Vec<KeyFrame>>, Error> {
    let (gltf, buffers, _) = gltf::import("E:/repos/Game-in-rust/blender_models/player_05.glb")?;


    let mut joints_indexes: std::collections::HashMap::<String, usize> = std::collections::HashMap::new();

    for i in 0..skeleton.joints.len() {
        joints_indexes.insert(skeleton.joints[i].name.clone(), i);
    }

    for ani in gltf.animations() {
        println!("ANIMATION {:#?}", ani.name());
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

                            let q = na::Quaternion::from_vector(na::Vector4::new(r[0], r[1], r[2], r[3]));

                            frames[i].joints[joints_index].rotation = na::UnitQuaternion::from_quaternion(q);
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
            name: "Empty".to_string(),
            duration: 1.0,
            skeleton: Skeleton {
                name: "Empty".to_string(),
                joints: Vec::new()
            },
            key_frames: Vec::new(),
        }
    }

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

        //println!("Frame {:#?}", keyframe);

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

            let local_matrix  = self.skeleton.joints[i].get_local_matrix_data(rotation, translation);

            let mut world_matrix = local_matrix;

            let parent_index = self.skeleton.joints[i].parent_index;
            if parent_index  != 255 {
                world_matrix = world_matrices[parent_index] * local_matrix;
            }

            world_matrices.push(world_matrix);

            self.skeleton.joints[i].world_matrix = world_matrix;
            bones[i] = world_matrix * self.skeleton.joints[i].inverse_bind_pose;

        }
    }
}
