use nalgebra as na;
use std::collections::{HashMap};

use crate::render_gl::{Skeleton, Joint};

use crate::resources;


#[derive(Debug, Clone)]
pub struct KeyframeAnimation {
    pub duration: f32,
    pub key_frames: Vec<KeyFrame>,
    pub cyclic: bool,
    pub root_motion: Option<na::Vector3::<f32>>,
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
    pub attack: KeyframeAnimation,
    pub attack_follow: KeyframeAnimation,
    pub t_pose: KeyframeAnimation,
    pub idle: KeyframeAnimation,
    pub roll: KeyframeAnimation,
}


pub fn load_animations(file_path: &str, skeleton: &Skeleton, base_animations: Option<&PlayerAnimations>) -> Result<PlayerAnimations, Error> {

    let animations = key_frames_from_gltf(file_path, skeleton)?;

    // This is not the FPS is will be played back at, by used to normalise longer and shorter animaions
    // to be invariant of keyframes

    let t_pose_frames = animations.get("t_pose");
    let walk_frames = animations.get("walk");
    let idle_frames = animations.get("idle");
    let attack_frames = animations.get("attack");
    let attack_follow_frames = animations.get("attack_follow");

    let mut roll_frames = animations.get("roll");

    let t_pose = create_animation(t_pose_frames, base_animations.map(|fb| &fb.t_pose), true);
    let walk = create_animation(walk_frames, base_animations.map(|fb| &fb.walk), true);
    let idle = create_animation(idle_frames, base_animations.map(|fb| &fb.idle), true);
    let attack = create_root_motion_animation(attack_frames, base_animations.map(|fb| &fb.attack), false);
    let attack_follow = create_animation(attack_follow_frames, base_animations.map(|fb| &fb.attack_follow), false);


    let roll = create_root_motion_animation(roll_frames, base_animations.map(|fb| &fb.roll), false);

    Ok(PlayerAnimations {
        t_pose,
        walk,
        idle,
        attack,
        attack_follow,
        roll
    })
}


fn create_root_motion_animation(frames: Option<&Vec::<KeyFrame>>, fall_back: Option<&KeyframeAnimation>, cyclic: bool) -> KeyframeAnimation {
    let frame_normalize = 40.0;


    match frames {
        Some(fs) => {
            let mut new_frames = fs.clone();
            // get root_motion into vec
            // remove movement from animation
            // so playing it without movement results in inplace animaiton
            let base = new_frames[0].joints[0].translation;

            let root_motion = (&mut new_frames).last().unwrap().joints[0].translation  - base;
            for frame in new_frames.iter_mut() {
                frame.joints[0].translation.x = 0.0;
                frame.joints[0].translation.y = 0.0;
            }

            KeyframeAnimation::new(new_frames.len() as f32 / frame_normalize, new_frames.clone(), cyclic, Some(root_motion))

        },
        None => fall_back.unwrap().clone(),
    }
}

fn create_animation(frames: Option<&Vec::<KeyFrame>>, fall_back: Option<&KeyframeAnimation>, cyclic: bool,) -> KeyframeAnimation {
    let frame_normalize = 40.0;

    match frames {
        Some(fs) => {
            KeyframeAnimation::new(fs.len() as f32 / frame_normalize, fs.clone(), cyclic, None)
        },
        None => fall_back.unwrap().clone(),
    }
}

fn key_frames_from_gltf(file_path: &str, skeleton: &Skeleton) -> Result<HashMap<String, Vec<KeyFrame>>, Error> {
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
            cyclic: true,
            duration: 1.0,
            key_frames: Vec::new(),
            root_motion: None
        }
    }

    pub fn new(duration: f32,  key_frames: Vec<KeyFrame>, cyclic: bool, root_motion: Option<na::Vector3::<f32>>) -> KeyframeAnimation {
        KeyframeAnimation {
            cyclic,
            duration,
            key_frames,
            root_motion,
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


    pub fn update_skeleton_to_key_frame(&mut self, skeleton: &mut Skeleton, next_keyframe: usize, t: f32) {

        // interpolate joints new transformation
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

            Skeleton::update_joint_matrices(&mut skeleton.joints, i, rotation, translation)

        }
    }
}
