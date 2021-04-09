#[derive(Debug)]
pub struct Skeleton {
    pub name: String,
    pub joints: Vec<Joint>,
}

#[derive(Debug)]
pub struct Joint {
    pub name: String,
    pub parent_index: usize,

    pub inverse_bind_pose: na::Matrix4::<f32>,
    pub world_matrix: na::Matrix4::<f32>,

    pub rotation: na::UnitQuaternion::<f32>,
    pub translation: na::Vector3::<f32>

}

impl Joint {

    pub fn get_base_local_matrix(&self) -> na::Matrix4::<f32> {
        self.get_local_matrix(na::UnitQuaternion::identity(), na::Vector3::new(0.0, 0.0, 0.0))
    }

    pub fn get_local_matrix(&self, rot: na::UnitQuaternion::<f32>, trans: na::Vector3::<f32>) -> na::Matrix4::<f32> {
        let r =  self.rotation * rot;
        let rot_mat = r.to_homogeneous();

        let trans_mat = na::Matrix4::new_translation(&(self.translation + trans));

        trans_mat * rot_mat
    }
}


impl Skeleton {
    pub fn from_collada(doc: &collada::document::ColladaDocument, name: &str) -> Skeleton {

        let name = " test".to_string();
        let mut joints = Vec::new();

        let skels = doc.get_skeletons().unwrap();

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

            return Skeleton {
                name,
                joints
            }
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
