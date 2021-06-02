use crate::render_gl::keyframe_animation::{Transformation};
use crate::render_gl::skeleton::{Skeleton, Joint};
use crate::entity;
use crate::math::*;


#[derive(Debug, Clone)]
pub struct IkLegs {
    pub left_leg: Ik,
    pub right_leg: Ik,
}

impl IkLegs {

    pub fn reset(&mut self) {
        self.right_leg.reset();
        self.left_leg.reset();
    }

    pub fn next_targets(&self) -> (Option<na::Vector3::<f32>>, Option<na::Vector3::<f32>>) {
        (self.left_leg.next_target(), self.right_leg.next_target())
    }

    pub fn ik_bones(&self) -> Vec::<usize> {


        //TODO optimize by storing at creating of IKLegs as hashset or something like that
        self.left_leg.bones.clone().into_iter().chain(self.right_leg.bones.clone().into_iter()).collect::<Vec<usize>>()
    }
}


#[derive(Debug, Clone)]
pub struct Ik {
    // maybe store a skeleton. or atleast the needed bones, Such that we can update the skeleton
    // This will enable easier forward kinematic

    // bones, in the order like "root" then children
    // mapping into a skeleton
    pub bones: Vec::<usize>,
    pub relative_target: na::Vector3::<f32>,
    pub pole: Transformation,
    a: f32,
    c: f32,
    state: IkState,
}


impl Ik {

    pub fn new(bones: Vec::<usize>, relative_target: na::Vector3::<f32>, pole: Transformation, joints: &Vec<Joint>) -> Ik{

        let hip_pos = Ik::joint_pos_internal(&bones, 0, joints).xz();
        let knee_pos = Ik::joint_pos_internal(&bones, 1, joints).xz();
        let foot_pos = Ik::joint_pos_internal(&bones, 2, joints).xz();


        // TODO can be precalculated, since our bones length does not change
        let a = (foot_pos - knee_pos).magnitude();
        let c = (hip_pos - knee_pos).magnitude();

        // set initial target to foot
        let target = Ik::joint_pos_internal(&bones, 2, joints);


        Ik {
            bones,
            relative_target: relative_target,
            pole,
            a,
            c,
            state: IkState::Rooted(target)
        }
    }


    pub fn current_target(&self) -> na::Vector3::<f32> {
        match self.state {
            IkState::Rooted(target) => target,
            IkState::Transition(ref trans) => trans.prev_target
        }
    }

    pub fn next_target(&self) -> Option<na::Vector3::<f32>> {
        match self.state {
            IkState::Rooted(_) => None,
            IkState::Transition(ref trans) => Some(trans.new_target)
        }
    }

    fn joint_pos_internal(bones: &Vec::<usize>, ik_index: usize, joints: &Vec<Joint>) -> na::Vector3::<f32> {

        na::Vector3::new(joints[bones[ik_index]].world_matrix[12],
                         joints[bones[ik_index]].world_matrix[13],
                         joints[bones[ik_index]].world_matrix[14])
    }

    pub fn joint_pos(&self, ik_index: usize, joints: &Vec<Joint>) -> na::Vector3::<f32> {

        Ik::joint_pos_internal(&self.bones, ik_index, joints)
    }

    pub fn reset(&mut self) {
        self.state = IkState::Rooted(self.relative_target);
    }
}

#[derive(Debug, Clone)]
pub enum IkSide {
    Left,
    Right
}


#[derive(Debug, Clone)]
pub enum IkState {
    Rooted(na::Vector3::<f32>),
    Transition(IkTransition)
}

#[derive(Debug, Clone)]
pub struct IkTransition {
    pub prev_target: na::Vector3::<f32>,
    pub new_target: na::Vector3::<f32>,
    pub physics_start_x: f32,
    pub t: f32
}



//REWORK THIS WALKING IK SO THAT ONE LEG START TRANSITION
//
pub fn update_ik(skeleton: &mut Skeleton, physics: &entity::Physics, delta: f32) {

    let ik_legs = match skeleton.legs {
        Some(ref mut ik_legs) => ik_legs,
        _ => {
            return;
        }
    };


    let left_pos = ik_legs.left_leg.joint_pos(2, &skeleton.joints).xz();
    let right_pos = ik_legs.right_leg.joint_pos(2, &skeleton.joints).xz();

    let mut left_can_trans = false;
    let mut right_can_trans = false;
    match (&ik_legs.left_leg.state, &ik_legs.right_leg.state) {
        (IkState::Rooted(_), IkState::Rooted(_)) => {
            // take the
            println!("BOTH ROOTEED {}" ,left_pos.x > right_pos.x);

            left_can_trans = left_pos.x < right_pos.x;
            right_can_trans = !left_can_trans

        },
        (IkState::Rooted(_), IkState::Transition(_)) => {
            println!("LEFT ROOT RIGHT TRANS");
            left_can_trans = false;
            right_can_trans = true;

        },
        (IkState::Transition(_), IkState::Rooted(_)) => {
            println!("LEFT TRANS RIGHT ROOT");
            left_can_trans = true;
            right_can_trans = false;

        },
        (IkState::Transition(_), IkState::Transition(_)) => {
            println!("BOTH TRANSITION");
            left_can_trans = true;
            right_can_trans = true;
        }


    }

    update_leg_ik(&mut ik_legs.left_leg, &mut skeleton.joints, physics, left_can_trans);
    update_leg_ik(&mut ik_legs.right_leg, &mut skeleton.joints, physics, right_can_trans);

    //println!("Left leg: {:.2?} Right leg: {:.2?}", left_pos, right_pos);

}


fn update_leg_ik(ik: &mut Ik, joints: &mut Vec<Joint>, physics: &entity::Physics, can_transition: bool) -> bool {

    let mut transition = false;
    match ik.state {
        IkState::Transition(ref mut trans) => {
            update_ik_t(trans, physics);
            transition = true;
        },
        _ => {}
    };

    update_ik_internal(ik, joints, physics, can_transition);

    transition
}

fn update_ik_t(ik_trans: &mut IkTransition, physics: &entity::Physics) {


    //OPTIMIZE: Maybe calc this one and store in IkTransition
    let plant_in_front_of_dist = 0.3;
    let total_dist = ik_trans.new_target.x - (ik_trans.physics_start_x + plant_in_front_of_dist);

    // 0 is
    let dist = ik_trans.new_target.x - (physics.pos.x + plant_in_front_of_dist);

    ik_trans.t = clamp01(total_dist - dist, 0.0, total_dist);

    //println!("{:?}, total_dist {}, dist {}", ik_trans.t, total_dist, dist);

}


fn update_ik_internal(ik: &mut Ik, joints: &mut Vec<Joint>, physics: &entity::Physics, can_transition: bool) {

    let mut hip_pos = (ik.joint_pos(0, joints) + physics.pos).xz();
    hip_pos.y -= 0.1;

    let angles;
    match ik.state {
        IkState::Rooted(target) => {
            // just calc the desired angles and of reach
            angles = get_angles(ik, hip_pos, target.xz());
            if angles.out_of_reach && can_transition {

                println!("TO TRANSITION");

                ik.state = IkState::Transition(IkTransition {
                    t: 0.0,
                    new_target: ik.relative_target + physics.pos,
                    prev_target: target,
                    physics_start_x: physics.pos.x,
                });
            }
        },
        IkState::Transition(ref trans) => {
            // mybe just update t here??

            let inter = trans.prev_target * (1.0 - trans.t) + (trans.new_target * trans.t);
            let arc_z = -trans.t*trans.t + trans.t;

            let x = inter.x;
            let z = inter.z + arc_z;

            //println!("{:?} {}", arc_y, trans.t);

            let inter_target = na::Vector2::new(x, z);

            //update_ik_t(trans, physics);
            angles = get_angles(ik, hip_pos, inter_target);
            //println!("t={:?} prev_x={} new_x = {} x = {}", trans.t, trans.prev_target.x, trans.new_target.x, angles.angle_knee.to_degrees());


            let angles_new_target = get_angles(ik, hip_pos, trans.new_target.xz());
            if trans.t >= 1.0 && !angles_new_target.out_of_reach {
                ik.state = IkState::Rooted(trans.new_target);
            }
        }
    };

    //println!("{:?}", angles.angle_knee.to_degrees());


    update_joint(ik, 0, angles.angle_hip, joints);
    update_joint(ik, 1, angles.angle_knee, joints);
    update_joint(ik, 2, 90.0_f32.to_radians(), joints);
}

pub struct IkAngles {
    angle_hip: f32,
    angle_knee: f32,
    out_of_reach: bool
}

fn get_angles(ik: &Ik, hip_pos: na::Vector2::<f32>, target: na::Vector2::<f32>) -> IkAngles {

    let hip_target_diff = target - hip_pos;

    let b = hip_target_diff.magnitude();

    let mut angle_hip = -90.0_f32.to_radians() - f32::atan2(hip_target_diff.y, hip_target_diff.x);

    let out_of_reach = b >= ik.a + ik.c;

    let mut beta = 0.0;
    if !out_of_reach {
        let alpha = f32::acos( (b*b + ik.c*ik.c - ik.a*ik.a) / (2.0 * b * ik.c));

        //println!("A B C {} {} {:?} {} {}", ik.a, b, ik.c, ik.a + ik.c, (b*b + ik.c*ik.c - ik.a*ik.a) / (2.0 * b * ik.c));
        //println!("FOOT, HIP {:?} {:?}", joints[ik.bones[2]], hip_pos);
        //println!("{:?} {} {} {}", alpha, ik.a, b, ik.c);
        angle_hip -= alpha;
        beta = - (std::f32::consts::PI - f32::acos( (ik.a*ik.a + ik.c*ik.c - b*b) / (2.0 * ik.a * ik.c)));
    }

    angle_hip = - angle_hip;

    // apply knee angle constrains

    let beta_new = f32::min(-10.0_f32.to_radians(), beta);
    angle_hip -= beta_new - beta;

    IkAngles {
        angle_hip,
        angle_knee: beta_new,
        out_of_reach
    }
}


fn update_joint(ik: &Ik, joint_index: usize, target_angle: f32, joints: &mut Vec<Joint>) {
    let mut i = ik.bones[joint_index];
    let euler = joints[i].rotation.euler_angles();

    let rotation = na::UnitQuaternion::from_euler_angles(target_angle, euler.1, euler.2);

    Skeleton::update_joint_matrices(joints, i, rotation, joints[i].translation);

}
