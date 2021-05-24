use crate::render_gl::keyframe_animation::{Transformation};
use crate::render_gl::skeleton::{Skeleton, Joint};
use crate::entity;


#[derive(Debug, Clone)]
pub struct Ik {
    // maybe store a skeleton. or atleast the needed bones, Such that we can update the skeleton
    // This will enable easier forward kinematic

    // bones, in the order like "root" then children
    // mapping into a skeleton
    pub bones: Vec::<usize>,
    pub current_target: Transformation,
    pub relative_target: Transformation,
    pub pole: Transformation,
    a: f32,
    c: f32,
}



impl Ik {

    pub fn new(bones: Vec::<usize>, target: Transformation, pole: Transformation, joints: &Vec<Joint>) -> Ik{

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
            pole,
            a,
            c
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

    /*
    println!("{:#?}", skeleton);
    panic!();
     */


    match skeleton.left_leg {
        Some(ref ik) => {
            update_ik_internal(ik, &mut skeleton.joints, physics, delta);
        },
        _ => {}
    };
}


fn update_ik_internal(ik: &Ik, joints: &mut Vec<Joint>, physics: &entity::Physics, delta: f32) {

    // For a leg angles 0,0,0 is a straightleg and foot
    // find the plane defined by target, hip, and pole
    // this 2d plane is the one we want to solve our Ik in

    let mut hip_pos = (ik.joint_pos(0, joints) + physics.pos).xz();

    let target = ik.current_target.translation.xz();

    let hip_target_diff = target - hip_pos;

    let b = hip_target_diff.magnitude();

    let mut angle_hip = -90.0_f32.to_radians() - f32::atan2(hip_target_diff.y, hip_target_diff.x);

    let out_of_reach = b >= ik.a + ik.c;

    //println!("HIP_angle {} {:?} {} ", angle_hip.to_degrees(), hip_target_diff, hip_target_diff.magnitude() > b);

    let mut beta = 0.0;
    if !out_of_reach {

        let alpha = f32::acos( (b*b + ik.c*ik.c - ik.a*ik.a) / (2.0 * b * ik.c));

        //println!("A B C {} {} {:?} {} {}", ik.a, b, ik.c, ik.a + ik.c, (b*b + ik.c*ik.c - ik.a*ik.a) / (2.0 * b * ik.c));
        //println!("FOOT, HIP {:?} {:?}", joints[ik.bones[2]], hip_pos);
        //println!("{:?} {} {} {}", alpha, ik.a, b, ik.c);
        angle_hip -= alpha;
        beta = - ( std::f32::consts::PI - f32::acos( (ik.a*ik.a + ik.c*ik.c - b*b) / (2.0 * ik.a * ik.c)));
    }
    //println!("HIP_angle {} {:?} {} ", angle_hip.to_degrees(), hip_target_diff, hip_target_diff.magnitude() > b);

    let mut i = ik.bones[0];
    let euler = joints[i].rotation.euler_angles();
    let rotation_0 = na::UnitQuaternion::from_euler_angles(-angle_hip, euler.1, euler.2);
    //let rotation_0 = na::UnitQuaternion::from_euler_angles(0.0_f32.to_radians(), euler.1, euler.2);
    Skeleton::update_joint_matrices(joints, i, rotation_0, joints[i].translation);

    i = ik.bones[1];
    let euler = joints[i].rotation.euler_angles();

    println!("BETA {}", beta);
    let rotation_1 = na::UnitQuaternion::from_euler_angles( beta, euler.1, euler.2);
    Skeleton::update_joint_matrices(joints, i, rotation_1, joints[i].translation);


    i = ik.bones[2];
    Skeleton::update_joint_matrices(joints, i, joints[i].rotation, joints[i].translation);


}
