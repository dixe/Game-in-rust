use crate::render_gl::{Transformation};

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub name: String,
    pub joints: Vec<Joint>,
}

#[derive(Debug, Clone)]
pub struct Joint {
    pub name: String,
    pub parent_index: usize,

    pub inverse_bind_pose: na::Matrix4::<f32>,
    pub world_matrix: na::Matrix4::<f32>,

    pub rotation: na::UnitQuaternion::<f32>,
    pub translation: na::Vector3::<f32>

}

impl Joint {

    pub fn get_local_matrix(&self) -> na::Matrix4::<f32> {
        let rot_mat = self.rotation.to_homogeneous();

        let trans_mat = na::Matrix4::new_translation(&self.translation);

        trans_mat * rot_mat
    }

    pub fn get_local_matrix_data(&self, rotation: na::UnitQuaternion::<f32>, translation: na::Vector3::<f32>) -> na::Matrix4::<f32> {
        let rot_mat = rotation.to_homogeneous();

        let trans_mat = na::Matrix4::new_translation(&translation);

        trans_mat * rot_mat
    }

    pub fn transformation(&self) -> Transformation {
        Transformation {
            rotation: self.rotation,
            translation: self.translation
        }
    }
}


impl Skeleton {

    fn calc_t_pose(&mut self) {
        for i in 0..self.joints.len() {
            self.set_t_pose_joint(i);
        }
    }

    fn set_t_pose_joint(&mut self, index: usize) {

        let joint = &self.joints[index];

        let local_matrix = joint.get_local_matrix();

        let mut world_matrix = local_matrix;

        if joint.parent_index != 255 {
            world_matrix = self.joints[joint.parent_index].world_matrix * local_matrix;
        }

        if joint.parent_index >= index && joint.parent_index != 255 {
            panic!("Bones are not in correct order. All children should be after parent current {}, parent {}", index, joint.parent_index);
        }

        println!("Index: {} - name: {}", index, joint.name.clone());
        //println!("name: {} worldmat :{:#?}", joint.name.clone(), world_matrix);
        //println!("name: {} inver_inverseworldmat :{:#?}", joint.name.clone(), world_matrix);

        let name = joint.name.clone();


        self.joints[index].world_matrix = world_matrix;
        self.joints[index].inverse_bind_pose = world_matrix.try_inverse().unwrap();

    }




    pub fn from_collada(doc: &collada::document::ColladaDocument, name: &str) -> Skeleton {

        let name = " test".to_string();
        let mut joints = Vec::new();

        let skel_res = doc.get_skeletons();


        let skels = match skel_res {
            Some(s) => s,
            None => {
                println!("Could not find skeleton");
                return Skeleton {
                    name,
                    joints
                }

            }
        };

        for skel in skels {

            let mut bind_poses = Vec::new();
            for bind_pose in &skel.bind_poses {
                bind_poses.push(map_mat4(bind_pose));
            }


            println!("poses: {:#?}", bind_poses.len());

            println!("skel bones: {:#?}", skel.joints.len());

            let mut index = 0;
            for joint in &skel.joints {

                let transform = bind_poses[index];
                let translation = na::Vector3::new(transform[12], transform[13], transform[14]);

                let mut rot_mat = na::Matrix3::identity();

                // take bind pose and remove all translation, giving us rotation
                rot_mat[0] = transform[0];
                rot_mat[1] = transform[1];
                rot_mat[2] = transform[2];

                rot_mat[3] = transform[4];
                rot_mat[4] = transform[5];
                rot_mat[5] = transform[6];

                rot_mat[6] = transform[8];
                rot_mat[7] = transform[9];
                rot_mat[8] = transform[10];

                let rotation = na::UnitQuaternion::from_matrix(&rot_mat);


                //println!("name = {} rotation = {:#?}", joint.name.clone(), rotation);

                //println!("name = {} transform = {:#?}", joint.name.clone(), transform);


                joints.push(Joint {
                    world_matrix: na::Matrix4::identity(),
                    name: joint.name.clone(),
                    parent_index: joint.parent_index as usize,
                    inverse_bind_pose: map_mat4(&joint.inverse_bind_pose),
                    rotation,
                    translation,

                });

                index +=1;
            }


            let mut skel =  Skeleton {
                name,
                joints
            };

            skel.calc_t_pose();

            return skel
        }


        panic!("No skeletons");

    }
}


fn map_mat4(col_mat: &collada::Matrix4<f32>) -> na::Matrix4::<f32> {

    let mut res = na::Matrix4::<f32>::identity();

    let mut index = 0;

    for i in 0..4 {
        for j in 0..4 {
            res[j*4 + i] = col_mat[i][j];
        }
    }

    res
}
