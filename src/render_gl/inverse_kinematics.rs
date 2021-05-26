use crate::render_gl::keyframe_animation::{Transformation};
use crate::render_gl::skeleton::{Skeleton, Joint};
use crate::entity;
use crate::math::*;

#[derive(Debug, Clone)]
pub struct Ik {
    // maybe store a skeleton. or atleast the needed bones, Such that we can update the skeleton
    // This will enable easier forward kinematic

    // bones, in the order like "root" then children
    // mapping into a skeleton
    pub bones: Vec::<usize>,
    pub current_target: na::Vector3::<f32>,
    pub next_target: na::Vector3::<f32>,
    pub relative_target: na::Vector3::<f32>,
    pub pole: Transformation,
    a: f32,
    c: f32,
    t: f32,
    out_of_reach: bool,
}




impl Ik {

    pub fn new(bones: Vec::<usize>, target: na::Vector3::<f32>, pole: Transformation, joints: &Vec<Joint>) -> Ik{

        let hip_pos = Ik::joint_pos_internal(&bones, 0, joints).xz();
        let knee_pos = Ik::joint_pos_internal(&bones, 1, joints).xz();
        let foot_pos = Ik::joint_pos_internal(&bones, 2, joints).xz();


        // TODO can be precalculated, since our bones length does not change
        let a = (foot_pos - knee_pos).magnitude();
        let c = (hip_pos - knee_pos).magnitude();


        Ik {
            bones,
            relative_target: target,
            current_target: target,
            next_target: target,
            pole,
            a,
            c,
            t: 1.0,
            out_of_reach: true
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
}


pub enum IkSide {
    Left,
    Right
}



pub fn update_ik(skeleton: &mut Skeleton, physics: &entity::Physics, delta: f32) {

    match skeleton.left_leg {
        Some(ref mut ik) => {
            update_ik_t(ik, physics, delta);

            update_ik_internal(ik, &mut skeleton.joints, physics);

            update_targets(ik, physics, delta);

        },
        _ => {}
    };
}


fn update_ik_t(ik: &mut Ik,  physics: &entity::Physics, delta: f32) {

    let total_dist = ik.relative_target.x;

    let plant_in_front_of_dist = 0.3;
    let dist = ik.next_target.x - (physics.pos.x + plant_in_front_of_dist);

    let change = ik.out_of_reach && dist < 0.0 && ik.t >= 1.0;
    ik.t = clamp01(total_dist - dist, 0.0, total_dist);
    println!("{:?}", ik.t);

}

fn update_targets(ik: &mut Ik,  physics: &entity::Physics, delta: f32) {

    let targets_dist = (ik.relative_target + physics.pos - ik.current_target).magnitude();

    // T shoul be based on distance traveled

    // t sohuld be 0 when next_target us right under us.
    // So take the x distance from relative target to player. this is just relative target dist
    // turn this distance into a range [0:1] and set t to that


    // we want to hit the floor when next_target.x == physics.pos.x

    let change = ik.out_of_reach && ik.t >= 1.0;

    //println!("TOTAL {:?} CURRENT {} T: {}", total_dist, dist, ik.t);

    //println!("{:?}", ik.t);
    if change {
        ik.current_target = ik.next_target;
        ik.next_target = ik.relative_target + physics.pos;
        //println!("TOTAL {:?} CURRENT {} T: {}", total_dist, dist, ik.t);
        ik.t = 0.0;
        println!("TARGET UPDATE");

    }




}


fn update_ik_internal(ik: &mut Ik, joints: &mut Vec<Joint>, physics: &entity::Physics) {
    // For a leg angles 0,0,0 is a straightleg and foot
    // find the plane defined by target, hip, and pole
    // this 2d plane is the one we want to solve our Ik in


    let mut hip_pos = (ik.joint_pos(0, joints) + physics.pos).xz();

    let target = ik.current_target.xz();


    // interpolate target instead of interpolate between them
    // this way we can make the foot follow a desired arc


    let x = ik.current_target.x * (1.0 - ik.t) + ik.next_target.x * ik.t;

    let arc_y = -ik.t*ik.t + ik.t;
    let y = ik.current_target.y * (1.0 - ik.t) + ik.next_target.y * ik.t + arc_y ;
    let inter_target = na::Vector2::new(x, y);

    let angles_cur_target = get_angles(ik, hip_pos, ik.current_target.xz());

    let angles_next_target = get_angles(ik, hip_pos, ik.next_target.xz());

    ik.out_of_reach = if ik.t < 1.0 {
        angles_cur_target.out_of_reach
    }
    else {
        angles_next_target.out_of_reach
    };


    let angles = get_angles(ik, hip_pos, inter_target);

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

    IkAngles {
        angle_hip,
        angle_knee: beta,
        out_of_reach
    }
}


fn update_joint(ik: &Ik, joint_index: usize, target_angle: f32, joints: &mut Vec<Joint>) {
    let mut i = ik.bones[joint_index];
    let euler = joints[i].rotation.euler_angles();

    let rotation = na::UnitQuaternion::from_euler_angles(target_angle, euler.1, euler.2);

    Skeleton::update_joint_matrices(joints, i, rotation, joints[i].translation);

}
