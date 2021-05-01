use nalgebra as na;

use crate::physics::projection_collision::*;


//TODO store better with faces normal ect. maybe
#[derive(Debug, Clone)]
pub struct CollisionBox {

    pub name: String,

    pub v0: na::Vector3::<f32>,
    pub v1: na::Vector3::<f32>,
    pub v2: na::Vector3::<f32>,
    pub v3: na::Vector3::<f32>,
    pub v4: na::Vector3::<f32>,
    pub v5: na::Vector3::<f32>,
    pub v6: na::Vector3::<f32>,
    pub v7: na::Vector3::<f32>,
}


impl CollisionBox {

    pub fn new(center: na::Vector3::<f32>, rot: na::Rotation3::<f32>, scale: na::Matrix3::<f32> ) -> CollisionBox {

        CollisionBox {
            v0: rot * (scale * na::Vector3::new(-0.5, -0.5, -0.5)) + center,
            v1: rot * (scale * na::Vector3::new(0.5, -0.5, -0.5) ) + center,
            v2: rot * (scale * na::Vector3::new(0.5, 0.5, -0.5)) + center,
            v3: rot * (scale * na::Vector3::new(-0.5, 0.5, -0.5)) + center,
            v4: rot * (scale * na::Vector3::new(-0.5, -0.5, 0.5)) + center,
            v5: rot * (scale * na::Vector3::new(0.5, -0.5, 0.5)) + center,
            v6: rot * (scale * na::Vector3::new(0.5, 0.5, 0.5)) + center,
            v7: rot * (scale * na::Vector3::new(-0.5, 0.5, 0.5)) + center,
            name: "".to_string()
        }
    }

    pub fn from_mesh_data(vertices: &Vec<na::Vector3::<f32>>) -> CollisionBox {

        CollisionBox {
            v0: vertices[0],
            v1: vertices[1],
            v2: vertices[2],
            v3: vertices[3],
            v4: vertices[4],
            v5: vertices[5],
            v6: vertices[6],
            v7: vertices[7],
            name: "".to_string()
        }
    }

    pub fn make_transformed(&self, translation: na::Vector3::<f32>, rotation: na::UnitQuaternion::<f32>) -> CollisionBox {

        CollisionBox {
            v0:  rotation *  self.v0  + translation,
            v1:  rotation *  self.v1  + translation,
            v2:  rotation *  self.v2  + translation,
            v3:  rotation *  self.v3  + translation,
            v4:  rotation *  self.v4  + translation,
            v5:  rotation *  self.v5  + translation,
            v6:  rotation *  self.v6  + translation,
            v7:  rotation *  self.v7  + translation,
            name: "".to_string()
        }


    }


    fn vertices(&self) -> Vec<na::Vector3::<f32>> {
        vec![ self.v0, self.v1, self.v2, self.v3, self.v4, self.v5, self.v6, self.v7]
    }

    fn sat_axis(&self) -> Vec<na::Vector3::<f32>> {

        // sat axis are all face normals. Since it is a box opposite normals wil create same axis
        // so just take the 3 unique

        let s1 = self.v0 - self.v1;
        let s2 = self.v1 - self.v2;
        let s3 = self.v0 - self.v4;

        vec![
            // the normals of the 3 faces we care about
            s1.cross(&s2),
            s2.cross(&s3),
            s3.cross(&s1)
        ]
    }


    fn edges(&self) -> Vec<na::Vector3::<f32>> {

        // sat axis are all face normals. Since it is a box opposite normals wil create same axis
        // so just take the 3 unique
        vec![
            // bottom square
            self.v0 - self.v1,
            self.v1 - self.v2,
            self.v0 - self.v4,
        ]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AxisBox {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_z: f32,
    pub max_z: f32,
}


pub fn check_collision(box_1: &CollisionBox, box_2: &CollisionBox) -> bool {

    // first find axis aligned bounding box collision
    let axis_collision = axis_aligned_collision(box_1, box_2);

    if !axis_collision {
        return false;
    }


    let mut all_sat_axis = box_1.sat_axis();
    all_sat_axis.append(&mut box_2.sat_axis());


    for e_1 in box_1.edges() {
        for e_2 in box_2.edges() {
            if e_1 == e_2 {
                continue;
            }
            let axis = e_1.cross(&e_2);

            all_sat_axis.push(axis);
        }
    }


    let mut has_gap = false;

    let vertices_1 = box_1.vertices();
    let vertices_2 = box_2.vertices();

    for axis in &all_sat_axis {

        let mut shape_1_max = vertices_1[0].dot(&axis);
        let mut shape_1_min = vertices_1[0].dot(&axis);
        for v in &vertices_1 {
            let proj_dot = projection(v, &axis).dot(&axis);

            shape_1_max = f32::max(shape_1_max, proj_dot);
            shape_1_min = f32::min(shape_1_min, proj_dot);
        }


        let mut shape_2_max = vertices_2[0].dot(&axis);
        let mut shape_2_min = vertices_2[0].dot(&axis);
        for v in &vertices_2 {
            let proj_dot = projection(v, &axis).dot(&axis);
            shape_2_max = f32::max(shape_2_max, proj_dot);
            shape_2_min = f32::min(shape_2_min, proj_dot);
        }

        has_gap = shape_1_min >= shape_2_max || shape_2_min >= shape_1_max;

        if has_gap {
            return false;
        }
    }

    !has_gap

}


fn axis_aligned_collision(box_1: &CollisionBox, box_2: &CollisionBox) -> bool {

    let ab_1 = create_axis_aligned_box(box_1);
    let ab_2 = create_axis_aligned_box(box_2);

    // do simple collision


    let res = (ab_1.min_x <= ab_2.max_x && ab_1.max_x >= ab_2.min_x) &&
        (ab_1.min_y <= ab_2.max_y && ab_1.max_y >= ab_2.min_y) &&
        (ab_1.min_z <= ab_2.max_z && ab_1.max_z >= ab_2.min_z);

    res
}

fn create_axis_aligned_box(cb: &CollisionBox) -> AxisBox {

    let min_x = f32::min(cb.v0.x, f32::min(cb.v1.x, f32::min(cb.v2.x, f32::min(cb.v3.x, f32::min(cb.v4.x, f32::min(cb.v5.x, f32::min(cb.v6.x, cb.v7.x)))))));

    let max_x = f32::max(cb.v0.x, f32::max(cb.v1.x, f32::max(cb.v2.x, f32::max(cb.v3.x, f32::max(cb.v4.x, f32::max(cb.v5.x, f32::max(cb.v6.x, cb.v7.x)))))));

    let min_y = f32::min(cb.v0.y, f32::min(cb.v1.y, f32::min(cb.v2.y, f32::min(cb.v3.y, f32::min(cb.v4.y, f32::min(cb.v5.y, f32::min(cb.v6.y, cb.v7.y)))))));

    let max_y = f32::max(cb.v0.y, f32::max(cb.v1.y, f32::max(cb.v2.y, f32::max(cb.v3.y, f32::max(cb.v4.y, f32::max(cb.v5.y, f32::max(cb.v6.y, cb.v7.y)))))));


    let min_z = f32::min(cb.v0.z, f32::min(cb.v1.z, f32::min(cb.v2.z, f32::min(cb.v3.z, f32::min(cb.v4.z, f32::min(cb.v5.z, f32::min(cb.v6.z, cb.v7.z)))))));
    let max_z = f32::max(cb.v0.z, f32::max(cb.v1.z, f32::max(cb.v2.z, f32::max(cb.v3.z, f32::max(cb.v4.z, f32::max(cb.v5.z, f32::max(cb.v6.z, cb.v7.z)))))));


    AxisBox {
        min_x,
        max_x,
        min_y,
        max_y,
        min_z,
        max_z,
    }
}




#[cfg(test)]

mod tests {

    use crate::physics::collision_3d::*;
    use nalgebra as na;


    fn create_box(off_set: na::Vector3::<f32>, rotation: Option<na::Vector3::<f32>>) -> CollisionBox {
        let rot_mat = match rotation {
            Some(rot) => na::Rotation3::new(rot),
            None => na::Rotation3::new(na::Vector3::new(0.0,0.0,1.0)),
        };

        // println!("{:#?}", rot_mat);


        CollisionBox {
            v0: rot_mat * na::Vector3::new(0.0, 0.0, 0.0) + off_set,
            v1: rot_mat * na::Vector3::new(1.0, 0.0, 0.0) + off_set,
            v2: rot_mat * na::Vector3::new(1.0, 1.0, 0.0) + off_set,
            v3: rot_mat * na::Vector3::new(0.0, 1.0, 0.0) + off_set,
            v4: rot_mat * na::Vector3::new(0.0, 0.0, 1.0) + off_set,
            v5: rot_mat * na::Vector3::new(1.0, 0.0, 1.0) + off_set,
            v6: rot_mat * na::Vector3::new(1.0, 1.0, 1.0) + off_set,
            v7: rot_mat * na::Vector3::new(0.0, 1.0, 1.0) + off_set,
        }
    }

    #[test]
    fn no_collision_axis() {

        let box_1 = create_box(na::Vector3::new(0.0, 0.0, 0.0), None);
        let box_2 = create_box(na::Vector3::new(1.2, 0.0, 0.0), None);

        let col = check_collision(&box_1, &box_2);
        println!("{} should be {}", col, false);

        assert!(!col);
    }

    #[test]
    fn collision_axis() {

        let box_1 = create_box(na::Vector3::new(0.0, 0.0, 0.0), None);
        let box_2 = create_box(na::Vector3::new(0.9, 0.0, 0.0), None);

        let col = check_collision(&box_1, &box_2);
        println!("{} should be {}", col, true);

        assert!(col);
    }

    #[test]
    fn collision_rot() {

        let rotation = na::Vector3::new(10.0_f32.to_radians(), 45.0_f32.to_radians(), 80.0_f32.to_radians());

        let box_1 = create_box(na::Vector3::new(0.0, 0.0, 0.0), None);
        let box_2 = create_box(na::Vector3::new(1.3, 0.0, 0.0), Some(rotation));

        let col = check_collision(&box_1, &box_2);
        println!("{} should be {}", col, true);


        assert!(col);
    }



}
