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



#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub v0: na::Vector3::<f32>,
    pub v1: na::Vector3::<f32>,
    pub v2: na::Vector3::<f32>,
    pub normal: na::Vector3::<f32>,
    pub d : f32,
}


pub struct EdgeWithNormal {
    pub v0: na::Vector3::<f32>,
    pub v1: na::Vector3::<f32>,
    pub normal: na::Vector3::<f32>,
}

impl Triangle {
    fn edges(&self) -> Vec<(na::Vector3::<f32>, na::Vector3::<f32>)> {
        vec! [
            (self.v0, self.v1),
            (self.v1, self.v2),
            (self.v2, self.v0),
        ]
    }

    fn edge_normals(&self) -> Vec<EdgeWithNormal> {

        vec! [
            EdgeWithNormal {
                v0: self.v0,
                v1: self.v2,
                normal: (self.v0 - self.v2).normalize().cross(&self.normal),
            },
            EdgeWithNormal {
                v0: self.v1,
                v1: self.v2,
                normal: (self.v2 - self.v1).normalize().cross(&self.normal),
            },
            EdgeWithNormal {
                v0: self.v1,
                v1: self.v0,
                normal: (self.v1 - self.v0).normalize().cross(&self.normal),
            },
        ]
    }
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

        let center = (self.v0 + self.v1 + self.v2 + self.v3 + self.v4 + self.v5 + self.v6 + self.v7) / 8.0;

        //println!("v0 ROT: {:?}", rotation *  (self.v7 - center)  + translation + center);
        CollisionBox {
            v0:  rotation *  (self.v0 - center)  + translation + center,
            v1:  rotation *  (self.v1 - center)  + translation + center,
            v2:  rotation *  (self.v2 - center)  + translation + center,
            v3:  rotation *  (self.v3 - center)  + translation + center,
            v4:  rotation *  (self.v4 - center)  + translation + center,
            v5:  rotation *  (self.v5 - center)  + translation + center,
            v6:  rotation *  (self.v6 - center)  + translation + center,
            v7:  rotation *  (self.v7 - center)  + translation + center,
            name: "".to_string()
        }
    }


    // TODO maybe put into trait vertices, where we just implement vertices() for each obj
    // then we can also get this info for triangle and more
    pub fn max_x(&self) -> f32 {

        let mut max = self.vertices()[0].x;
        for v in &self.vertices() {
            max = f32::max(max, v.x);
        }

        max
    }

    pub fn min_x(&self) -> f32 {

        let mut min = self.vertices()[0].x;
        for v in &self.vertices() {
            min = f32::min(min, v.x);
        }

        min
    }

    pub fn max_y(&self) -> f32 {

        let mut max = self.vertices()[0].y;
        for v in &self.vertices() {
            max = f32::max(max,v.y);
        }

        max
    }

    pub fn min_y(&self) -> f32 {

        let mut min = self.vertices()[0].y;
        for v in &self.vertices() {
            min = f32::min(min, v.y);
        }

        min
    }

    pub fn max_z(&self) -> f32 {

        let mut max = self.vertices()[0].z;
        for v in &self.vertices() {
            max = f32::max(max,v.z);
        }

        max
    }

    pub fn min_z(&self) -> f32 {

        let mut min = self.vertices()[0].z;
        for v in &self.vertices() {
            min = f32::min(min, v.z);
        }

        min
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
            s1.cross(&s2).normalize(),
            s2.cross(&s3).normalize(),
            s3.cross(&s1).normalize()
        ]
    }

    fn edges(&self) -> Vec<(na::Vector3::<f32>, na::Vector3::<f32>)> {

        vec![
            // the normals of the 3 faces we care about
            // BOTTOM
            (self.v0, self.v1),
            (self.v1, self.v2),
            (self.v2, self.v3),
            (self.v3, self.v0),

            //(TOP
            (self.v4, self.v5),
            (self.v5, self.v6),
            (self.v6, self.v7),
            (self.v7, self.v4),

            // (SIDES
            (self.v0, self.v4),
            (self.v1, self.v5),
            (self.v2, self.v6),
            (self.v3, self.v7),
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


pub enum CollisionResult {
    NoCollision,
    Collision(na::Vector3::<f32>),
}

impl CollisionResult {
    pub fn has_collision(&self) -> bool {
        match self {
            CollisionResult::NoCollision => false,
            _ => true
        }
    }
}

pub fn check_collision(box_1: &CollisionBox, box_2: &CollisionBox) -> CollisionResult  {

    // first find axis aligned bounding box collision
    let axis_collision = axis_aligned_collision(box_1, box_2);

    if !axis_collision {
        return CollisionResult::NoCollision;
    }


    let mut all_sat_axis = box_1.sat_axis();
    all_sat_axis.append(&mut box_2.sat_axis());


    let mut has_gap = false;

    let vertices_1 = box_1.vertices();
    let vertices_2 = box_2.vertices();


    let mut smallest_overlap = 10000000.0;
    let mut smallest_overlap_dir = na::Vector3::new(0.0, 0.0, 0.0);
    let mut below = false;
    for axis in &all_sat_axis {

        match sat_inner(&vertices_1, &vertices_2, &axis) {
            None => {
                return CollisionResult::NoCollision;
            },
            Some(inner_res) => {

                if inner_res.dist < smallest_overlap {
                    below = inner_res.below;
                    smallest_overlap = inner_res.dist;
                    smallest_overlap_dir = *axis;
                }
            }
        }
    }

    if below {
        smallest_overlap_dir *= -1.0;
    }

    CollisionResult::Collision(smallest_overlap * smallest_overlap_dir)

}

#[derive(Clone, Debug, Copy)]
struct InnerRes {
    dist: f32,
    below: bool
}

fn sat_inner(vertices_1: &[na::Vector3::<f32>], vertices_2: &[na::Vector3::<f32>], axis: &na::Vector3::<f32>) -> Option<InnerRes> {

    let mut shape_1_max = vertices_1[0].dot(axis);
    let mut shape_1_min = vertices_1[0].dot(axis);
    for v in vertices_1 {
        let proj_dot = projection(v, &axis).dot(&axis);

        shape_1_max = f32::max(shape_1_max, proj_dot);
        shape_1_min = f32::min(shape_1_min, proj_dot);
    }


    let mut shape_2_max = vertices_2[0].dot(&axis);
    let mut shape_2_min = vertices_2[0].dot(&axis);
    for v in vertices_2 {
        let proj_dot = projection(v, &axis).dot(&axis);
        shape_2_max = f32::max(shape_2_max, proj_dot);
        shape_2_min = f32::min(shape_2_min, proj_dot);
    }

    let has_gap = shape_1_min >= shape_2_max || shape_2_min >= shape_1_max;

    if has_gap {
        return None
    }

    let dist = f32::min(shape_1_max - shape_2_min, shape_2_max - shape_1_min);

    Some(InnerRes {dist, below: shape_1_max - shape_2_min > shape_2_max - shape_1_min })

}



pub fn check_collision_triangles(box_1: &CollisionBox, triangles: &[Triangle]) -> CollisionResult {

    // triangles is not bound to give a convex shape, thus the logic is a bit different from
    // the two boxes case
    let box_vertices = box_1.vertices();
    let mut resolve_dir = na::Vector3::<f32>::new(0.0, 0.0, 0.0);
    let mut collision = false;

    //println!("START!");
    for triangle in triangles {
        match triangle_box_collision(&box_1, &triangle) {
            CollisionResult::NoCollision => {
            },
            CollisionResult::Collision(resolve) => {
                resolve_dir = resolve;
                collision = true;
            }
        };
    }

    //println!("END!");
    if !collision {
        CollisionResult::NoCollision
    }
    else
    {
        CollisionResult::Collision(resolve_dir)
    }
}


fn triangle_box_collision(box_1: &CollisionBox, triangle: &Triangle) -> CollisionResult {
    // Based on answer with 43 by
    // https://stackoverflow.com/a/17661431
    let mut sign_differ = Vec::new();
    for edge in box_1.edges() {

        let p0_negative = (edge.0.dot(&triangle.normal) + triangle.d).is_sign_negative();
        let p1_negative = (triangle.normal.dot(&edge.1) + triangle.d).is_sign_negative();

        if(p0_negative != p1_negative) {
            /*
            println!("Edge {:?}", edge);
            println!("Normal {:?}", triangle.normal);
            println!("DOT {:?}", edge.0.dot(&triangle.normal));
            println!("DOT {:?}", triangle.normal.dot(&edge.1));
            println!("D {:?}", triangle.d);
             */
            sign_differ.push(edge);
        }
    }

    // no sign differ means box did not intersect plane
    if sign_differ.len() == 0 {
        return CollisionResult::NoCollision;
    }


    // plane intersected, but we need to know if it was inside the triangle


    let mut correction = -1.0;
    for edge in sign_differ {
        //println!("DIFFER {:?}", edge);
        let l = edge.1 - edge.0;
        let l0 = edge.0;
        let p0 = triangle.v0;
        let d = (p0 - l0).dot(&triangle.normal) / (l.dot(&triangle.normal));

        let intersect_p = l0 + l * d;

        // println!("INTERSECT {:?}", intersect_p);

        // check if intersect_p is inside triangle.

        let mut inside = true;


        for edge_norm in triangle.edge_normals() {

            let inter_proj = projection(&intersect_p, &edge_norm.normal).dot(&edge_norm.normal);
            let mut edge_vertex_proj = projection(&edge_norm.v0, &edge_norm.normal).dot(&edge_norm.normal);

            let sign = edge_vertex_proj.signum();

            edge_vertex_proj = edge_vertex_proj;

            inside &= inter_proj <= edge_vertex_proj;

            /*
            if inter_proj > edge_vertex_proj {
            //println!("NOT INS {:?} dot {}", edge_norm, edge_norm.normalize().dot(&intersect_p.normalize()));
            println!("INSIDE EDGE POINTS {:?}", (edge_norm.v0, edge_norm.v1) );
            println!("INTER POINT {:?}", intersect_p);
            println!("EDGE NORMAL{:?}", edge_norm.normal);
            println!("PROJECTIONS {}  {} {}", sign, inter_proj, edge_vertex_proj);
            println!("");
        }
             */

        }

        if inside {
            // clear direction is normal, so find out which way is the shortest
            // between edge.0 and edge.1

            // project intersect, edge.0 and edge.1 onto normal
            // take distance intersect_projection - edge0_projection and the opther and take smalle one

            let base = projection(&intersect_p, &triangle.normal).magnitude();

            let e0 = projection(&edge.0, &triangle.normal).magnitude() - base;
            let e1 = projection(&edge.1, &triangle.normal).magnitude() - base;

            // take the one that is positive


            let pot_cor = f32::min(e0.abs(), e1.abs());


            //println!("e0, e1 pot_cor {:?} {} {}", e0, e1, pot_cor);

            if pot_cor > correction {
                correction = pot_cor;
            }
        }

    }

    if correction < 0.0 {
        return CollisionResult::NoCollision;
    }

    //println!("POT COR {}  NORMAL{:?}", correction, triangle.normal);
    CollisionResult::Collision(correction * triangle.normal)

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
            None => na::Rotation3::identity(),
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
            name: "".to_string()
        }
    }

    #[test]
    fn no_collision_axis() {

        let box_1 = create_box(na::Vector3::new(0.0, 0.0, 0.0), None);
        let box_2 = create_box(na::Vector3::new(1.2, 0.0, 0.0), None);

        let collision_res = check_collision(&box_1, &box_2);
        assert!(!collision_res.has_collision());

    }

    #[test]
    fn collision_axis() {

        let box_1 = create_box(na::Vector3::new(0.0, 0.0, 0.0), None);
        let box_2 = create_box(na::Vector3::new(0.9, 0.0, 0.0), None);

        let collision_res = check_collision(&box_1, &box_2);
        assert!(collision_res.has_collision());
    }

    #[test]
    fn collision_rot() {

        let rotation = na::Vector3::new(10.0_f32.to_radians(), 45.0_f32.to_radians(), 80.0_f32.to_radians());

        let box_1 = create_box(na::Vector3::new(0.0, 0.0, 0.0), None);
        let box_2 = create_box(na::Vector3::new(1.3, 0.0, 0.0), Some(rotation));

        let collision_res = check_collision(&box_1, &box_2);
        assert!(collision_res.has_collision());
    }

    #[test]
    fn correction_test_1() {

        let box_1 = create_box(na::Vector3::new(0.0, 0.0, 0.0), None);
        let box_2 = create_box(na::Vector3::new(0.9, 0.0, 0.0), None);

        let collision_res = check_collision(&box_1, &box_2);

        match collision_res {
            CollisionResult::Collision(resolve_dir) => {
                println!("\n{:?}\n\n", resolve_dir);
                let dot = resolve_dir.normalize().dot(&na::Vector3::new(1.0, 0.0, 0.0));
                println!("DOT {:#?}", dot);
                assert!(dot > 0.99);
                let depth = resolve_dir.magnitude();
                assert!( depth > 0.0 && depth < 0.2);

            },
            _ => {
                assert!(false);
            }
        };

    }


    #[test]
    fn correction_test_2() {

        let box_1 = create_box(na::Vector3::new(0.0, 0.0, 0.0), None);
        let box_2 = create_box(na::Vector3::new(-0.9, 0.0, 0.0), None);

        let collision_res = check_collision(&box_1, &box_2);

        match collision_res {
            CollisionResult::Collision(resolve_dir) => {
                println!("\n{:?}\n\n" , resolve_dir);
                let dot = resolve_dir.normalize().dot(&na::Vector3::new(1.0, 0.0, 0.0));
                println!("DOT {:#?}", dot);
                assert!(dot < -0.99);
                let depth = resolve_dir.magnitude();
                assert!( depth > 0.0 && depth < 0.2);

            },
            _ => {
                assert!(false);
            }
        };

    }


    #[test]
    fn triangles_1 () {


        let box_1 = create_box(na::Vector3::new(0.0, 0.0, 0.1), None);

        let triangles = vec![ Triangle {
            v0: na::Vector3::new(-0.5, -0.5, 0.0),
            v1: na::Vector3::new(0.5, -0.5, 0.0),
            v2: na::Vector3::new(0.0, 0.5, 0.0),
            normal: na::Vector3::new(0.0, 0.0, 1.0),
            d: - na::Vector3::new(-0.5, -0.5, 0.0).dot(&na::Vector3::new(0.0, 0.0, 1.0))
        },
        ];


        let collision_res = check_collision_triangles(&box_1, &triangles);

        match collision_res {

            CollisionResult::Collision(_) => {
                assert!(false);
            },
            _ => {
            }
        };
    }


    #[test]
    fn triangles_2 () {

        let box_1 = create_box(na::Vector3::new(0.0, 0.0, -0.1), None);

        let triangles = vec![ Triangle {
            v0: na::Vector3::new(-0.5, -0.5, 0.0),
            v1: na::Vector3::new(0.5, -0.5, 0.0),
            v2: na::Vector3::new(0.0, 0.5, 0.0),
            normal: na::Vector3::new(0.0, 0.0, 1.0),
            d: - na::Vector3::new(-0.5, -0.5, 0.0).dot(&na::Vector3::new(0.0, 0.0, 1.0))
        },
        ];


        let collision_res = check_collision_triangles(&box_1, &triangles);

        match collision_res {
            CollisionResult::Collision(resolve_vec) => {
                println!("\n\n {:?}\n\n" , resolve_vec);
                let dot = resolve_vec.normalize().dot(&na::Vector3::new(0.0, 0.0, -1.0));
                println!("DOT {:#?}", dot);

                assert!(dot < -0.99);
                let depth = resolve_vec.magnitude();
                println!("DEPTH {:#?}", depth);
                assert!( depth > 0.09 && depth < 1.1);

            },
            _ => {
                assert!(false);
            }
        };


    }

    #[test]
    fn triangle_box_0 () {

        let box_1 = create_box(na::Vector3::new(0.0, 0.0, 0.5), None);

        let triangle = Triangle {
            v0: na::Vector3::new(-0.5, -0.5, 0.0),
            v1: na::Vector3::new(0.5, -0.5, 0.0),
            v2: na::Vector3::new(0.0, 0.5, 0.0),
            normal: na::Vector3::new(0.0, 0.0, 1.0),
            d: - na::Vector3::new(-0.5, -0.5, 0.0).dot(&na::Vector3::new(0.0, 0.0, 1.0))
        };

        let col = triangle_box_collision(&box_1, &triangle);

        match col {
            CollisionResult::Collision(resolve_vec) => {
                assert!(false);
            },
            _ => {
                assert!(true);
            }
        };

    }

    #[test]
    fn triangle_box_1 () {

        let box_1 = create_box(na::Vector3::new(0.0, 0.0, -0.7), None);

        let triangle = Triangle {
            v0: na::Vector3::new(-0.5, -0.5, 0.0),
            v1: na::Vector3::new(0.5, -0.5, 0.0),
            v2: na::Vector3::new(0.0, 0.5, 0.0),
            normal: na::Vector3::new(0.0, 0.0, 1.0),
            d: - na::Vector3::new(-0.5, -0.5, 0.0).dot(&na::Vector3::new(0.0, 0.0, 1.0))
        };

        let col = triangle_box_collision(&box_1, &triangle);


        match col {
            CollisionResult::Collision(resolve_vec) => {
                let depth = resolve_vec.magnitude();
                println!("DEPTH {:#?}", depth);
                assert!( (0.3 -depth).abs() < 0.001);
            },
            _ => {
                assert!(false);
            }
        };
    }

    #[test]
    fn triangle_box_2 () {

        let box_1 = CollisionBox {
            name: "".to_string(),
            v0: na::Vector3::new(-27.76655, -4.172013, 0.0031699687),
            v1: na::Vector3::new(-28.096537, -4.4879, 0.0031699687),
            v2: na::Vector3::new(-27.78065, -4.817886, 0.0031699687),
            v3: na::Vector3::new(-27.450663, -4.501999, 0.0031699687),
            v4: na::Vector3::new(-27.76655, -4.172013, 1.8767198),
            v5: na::Vector3::new(-28.096537, -4.4879, 1.8767198),
            v6: na::Vector3::new(-27.78065, -4.817886, 1.8767198),
            v7: na::Vector3::new(-27.450663, -4.501999, 1.8767198)
        };

        let triangle = Triangle {
            v0: na::Vector3::new(-37.73467, 14.662499, 2.730679),
            v1: na::Vector3::new(-30.882118, -29.384829, 1.952274),
            v2: na::Vector3::new(-26.58799, 26.58799, 0.),
            normal: na::Vector3::new(0.24274935, 0.020625582, 0.9698698),
            d: 6.209241
        };

        let col = triangle_box_collision(&box_1, &triangle);


        match col {
            CollisionResult::Collision(resolve_vec) => {
                let depth = resolve_vec.magnitude();
                println!("DEPTH {:#?}", depth);
                assert!( (0.7 -depth).abs() < 0.001);
            },
            _ => {
                println!("no collision it say");
                assert!(false);
            }
        };
    }



    #[test]
    fn collision_box_transform() {
        let box_1 = create_box(na::Vector3::new(7.0, 6.1, -2.7), None);
        let box_1 = create_box(na::Vector3::new(0.0, 0.0, 0.0), None);

        let rotation = na::UnitQuaternion::<f32>::from_euler_angles(-0.0, -0.0, 3.028);
        let translation = na::Vector3::new(-18.63, -11.55, 0.0);

        let trans = box_1.make_transformed(translation, rotation);

        println!("box_1 max_x, min_x, max_y, min_y, max_z, min_z {} {} {} {} {} {}",
                 box_1.max_x(), box_1.min_x(),
                 box_1.max_y(), box_1.min_y(),
                 box_1.max_z(), box_1.min_z() );
        println!("");
        println!("trans max_x, min_x, max_y, min_y, max_z, min_z {} {} {} {} {} {}",
                 trans.max_x(), trans.min_x(),
                 trans.max_y(), trans.min_y(),
                 trans.max_z(), trans.min_z() );

        assert!(true);
    }


}
