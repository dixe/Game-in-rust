use nalgebra as na;
use std::ops::Index;


pub struct CollisionBox {
    pub pos: na::Vector3::<f32>,
    pub side_len: f32,
}

#[derive(Copy, Clone)]
pub struct Side {
    pub v1: na::Vector3::<f32>,
    pub v2: na::Vector3::<f32>
}


#[derive(Copy, Clone)]
pub struct NormalSide {
    pub w: na::Vector3::<f32>,
    pub b: na::Matrix1::<f32>

}

pub fn collision_side(vertices: Vec::<na::Vector3::<f32>>, side: &NormalSide) -> (bool, na::Vector3::<f32>) {

    // Kinda inspires ny how svm check if in category 0 or 1, with a hyperplane

    let mut crossing = false;

    let mut m_val_min = 1.0;

    for v in &vertices {

        let m: na::Matrix1::<f32>  = side.w.transpose() * v - side.b;
        let m_val: f32 = *m.index(0);


        m_val_min = f32::min(m_val, m_val_min);

        crossing |= m_val <= 0.0;
    }


    (crossing, side.w * m_val_min)
}


pub fn collision_sat(vertices: Vec::<na::Vector3::<f32>>, sides: &[Side]) -> (bool, na::Vector3::<f32>) {

    let vertices_1 = vertices;

    let vertices_2 = vertices_from_sides(&sides);
    let mut has_gap = false;

    let mut smallest_overlap = 10000000000000000000.0;
    let mut smallest_overlap_dir = na::Vector3::new(0.0, 0.0, 0.0);

    'sides: for s in sides {

        let line = (s.v1 - s.v2).normalize();

        let wall = na::Vector3::new( - line.y, line.x, line.z).normalize();

        let mut box_1_max = 0.0;
        let mut box_1_min = vertices_1[0].dot(&wall);
        for v in &vertices_1 {
            let proj_dot = projection(v, &wall).dot(&wall);

            box_1_max = f32::max(box_1_max, proj_dot);
            box_1_min = f32::min(box_1_min, proj_dot);
        }


        let mut box_2_max = 0.0;
        let mut box_2_min = vertices_2[0].dot(&wall);
        for v in &vertices_2 {
            let proj_dot = projection(v, &wall).dot(&wall);
            box_2_max = f32::max(box_2_max, proj_dot);
            box_2_min = f32::min(box_2_min, proj_dot);
        }


        let overlap = (box_1_min <= box_2_min && box_1_max >= box_2_min) ||
            (box_1_min <= box_2_max && box_1_max >= box_2_max);

        has_gap = !overlap;

        if has_gap {
            break 'sides;
        }


        let smaller = f32::min(box_1_max - box_2_min, box_2_max - box_1_min);
        if smaller < smallest_overlap {
            smallest_overlap = smaller;
            smallest_overlap_dir = wall;
        }

    }


    (!has_gap,  smallest_overlap_dir * smallest_overlap)

}

fn vertices_from_sides(sides: &[Side]) -> Vec::<na::Vector3::<f32>> {

    let mut r = Vec::<na::Vector3::<f32>>::new();

    for s in sides {
        r.push(s.v1);
    }

    r
}


pub fn generate_normal_side(v1: na::Vector3::<f32>, v2: na::Vector3::<f32>) -> NormalSide {
    let line = v2-v1;
    let w = na::Vector3::new(-line.y, line.x, line.z).normalize();

    let b = w.transpose() * v1;

    NormalSide{
        w,
        b
    }

}


pub fn generate_vertices(b: &CollisionBox) -> Vec::<na::Vector3::<f32>> {
    let v00 = na::Vector3::new(
        b.pos.x,
        b.pos.y,
        0.0);
    let v01 = na::Vector3::new(
        b.pos.x,
        b.pos.y + b.side_len,
        0.0);
    let v10 = na::Vector3::new(
        b.pos.x + b.side_len,
        b.pos.y,
        0.0);
    let v11 = na::Vector3::new(
        b.pos.x + b.side_len,
        b.pos.y + b.side_len,
        0.0);

    vec! [ v00, v01, v10, v11]

}


pub fn generate_sides(b: &CollisionBox) -> Vec::<Side> {
    let v00 = na::Vector3::new(
        b.pos.x,
        b.pos.y,
        0.0);
    let v01 = na::Vector3::new(
        b.pos.x,
        b.pos.y + b.side_len,
        0.0);
    let v10 = na::Vector3::new(
        b.pos.x + b.side_len,
        b.pos.y,
        0.0);
    let v11 = na::Vector3::new(
        b.pos.x + b.side_len,
        b.pos.y + b.side_len,
        0.0);

    let v =vec! [
        Side {v1: v00, v2: v10},
        Side {v1: v10, v2: v11},
        Side {v1: v11, v2: v01},
        Side {v1: v01, v2: v00}
    ];

    v
}


pub fn projection(from: &na::Vector3::<f32>, onto: &na::Vector3::<f32>) -> na::Vector3::<f32>  {
    (from.dot(onto) / onto.dot(onto)) * onto
}


#[cfg(test)]
mod tests {


    use crate::physics::projection_collision::{CollisionBox, projection, collision_sat, generate_vertices, generate_sides, collision_side, generate_normal_side };
    use nalgebra as na;

    #[test]
    fn test_projection_1() {


        let line =  na::Vector3::new(1.0, 0.0, 0.0);

        let vertex1 = na::Vector3::new(1.9, 1.0, 0.0);

        let proj1 = projection(&vertex1, &line);

        assert_eq!(proj1, na::Vector3::new(1.9, 0.0, 0.0));
    }


    #[test]
    fn collision_sat_intersect_1() {

        let box1 = CollisionBox {
            pos: na::Vector3::new(1.0, 0.8, 0.0),
            side_len: 1.0,

        };


        let box2 = CollisionBox {
            pos: na::Vector3::new(1.0, 0.0, 0.0),
            side_len: 1.0,

        };



        let (has_col, _) = collision_sat(generate_vertices(&box1), generate_sides(&box2).as_slice());

        assert!(has_col);

        //assert!(dir.magnitude() - 0.4 < 0.001);

    }

    #[test]
    fn collision_sat_intersect_2() {

        let box1 = CollisionBox {
            pos: na::Vector3::new(1.0, 0.0, 0.0),
            side_len: 1.0,

        };


        let box2 = CollisionBox {
            pos: na::Vector3::new(1.1, 0.0, 0.0),
            side_len: 1.0,

        };



        let (has_col, _) = collision_sat(generate_vertices(&box1), generate_sides(&box2).as_slice());

        assert!(has_col);

        //assert!(dir.magnitude() - 0.4 < 0.001);

    }

    #[test]
    fn collision_sat_no_intersect() {

        let box1 = CollisionBox {
            pos: na::Vector3::new(1.0, 0.0, 0.0),
            side_len: 1.0,

        };


        let box2 = CollisionBox {
            pos: na::Vector3::new(2.1, 0.0, 0.0),
            side_len: 1.0,

        };

        let (has_col, _) = collision_sat(generate_vertices(&box1), generate_sides(&box2).as_slice());

        assert!(!has_col);
    }


    #[test]
    fn collision_side_left_false() {

        let box1 = CollisionBox {
            pos: na::Vector3::new(2.0, -10.0, 0.0),
            side_len: 1.0,

        };

        let side = generate_normal_side(
            na::Vector3::new(-8.0, 9.0,0.0),
            na::Vector3::new(-8.0, -10.0,0.0),
        );

        let (has_col, _) = collision_side(generate_vertices(&box1), &side);

        assert!(!has_col);
    }

    #[test]
    fn collision_side_left_true() {

        let box1 = CollisionBox {
            pos: na::Vector3::new(-8.11, -0.96, 0.0),
            side_len: 1.0,

        };


        let side = generate_normal_side(
            na::Vector3::new(-8.0, 9.0,0.0),
            na::Vector3::new(-8.0, -10.0,0.0),
        );

        let (has_col, dir) = collision_side(generate_vertices(&box1), &side);

        println!("{}", dir);
        assert!(has_col);
    }

}
