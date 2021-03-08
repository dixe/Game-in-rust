use nalgebra as na;
use std::ops::Index;


pub struct CollisionBox {
    pub pos: na::Vector3::<f32>,
    pub side_len: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Side {
    pub v1: na::Vector3::<f32>,
    pub v2: na::Vector3::<f32>
}


#[derive(Copy, Clone, Debug)]
pub struct NormalSide {
    pub w: na::Vector3::<f32>,
    pub b: na::Matrix1::<f32>,
    pub v1: na::Vector3::<f32>,
    pub v2: na::Vector3::<f32>,

}

pub fn collision_side(vertices: Vec::<na::Vector3::<f32>>, side: &NormalSide) -> (bool, na::Vector3::<f32>) {

    let sides = generate_side(&vertices);

    let mut min_mag = 1000.0;
    let mut ret_correction = na::Vector3::<f32>::new(0.0, 0.0, 0.0);
    let mut has_col = false;

    for s in &sides {
        let (col, correction, mag) = collision_side_piece(s, side);
        if col {
            //println!("{:#?} {:#?}", s, side);
            has_col |= col;
            if mag < min_mag {

                min_mag = mag;
                ret_correction = correction;
            }
        }
    }


    (has_col, ret_correction)
}


fn collision_side_single(vertex: na::Vector3::<f32>, side: &NormalSide) -> (bool, na::Vector3::<f32>, f32) {

    // Kinda inspires ny how svm check if in category 0 or 1, with a hyperplane


    let m: na::Matrix1::<f32>  = side.w.transpose() * vertex - side.b;
    let m_val: f32 = *m.index(0);

    // avoid doing normalize on 0 vector
    //let norm = if m_val == 0.0 { na::Vector3::new(0.0, 0.0, 0.0)} else {side.w * m_val};
    (m_val <= 0.0,  side.w * m_val,  f32::abs(m_val))
}


fn can_collide(side: &Side, normal_side: &NormalSide) -> bool {


    // take side verticies and project down onto line, of they don't overlap, we are free

    let wall = normal_side.v2 - normal_side.v1;

    let side_proj_1 = projection(&side.v1, &wall).dot(&wall);
    let side_proj_2 = projection(&side.v2, &wall).dot(&wall);

    let s_min = f32::min(side_proj_1, side_proj_2);
    let s_max = f32::max(side_proj_1, side_proj_2);

    let normal_proj_1 = projection(&normal_side.v1, &wall).dot(&wall);
    let normal_proj_2 = projection(&normal_side.v2, &wall).dot(&wall);

    let normal_min = f32::min(normal_proj_1, normal_proj_2);
    let normal_max = f32::max(normal_proj_1, normal_proj_2);

    let can  = s_min < normal_max && s_max > normal_min;

    //println!("{}, {}, {}, {} {}", s_min, s_max, normal_min, normal_max, can);

    can
}


fn collision_side_piece(side: &Side, normal_side: &NormalSide) -> (bool, na::Vector3::<f32>, f32) {

    if !can_collide(side, normal_side) {
        return (false, normal_side.w, 1.0);
    }

    let (col1, correction1, mag_1) = collision_side_single(side.v1, normal_side);
    let (col2, correction2, mag_2) = collision_side_single(side.v2, normal_side);




    // if nothing overlaps we can stop, otherwise check how we overlap
    if !col1 && !col2 {
        return (false, correction1, mag_1);
    }

    //make two triangle:
    //t1; side.v1, side.v2, normal_side.v1
    //t2: side.v1, side.v2, normal_side.v2

    let vertices_normal = generate_normal_side(side.v2, side.v1);

    let (col_n_1, d1, _) = collision_side_single(normal_side.v1, &vertices_normal);
    let (col_n_2, d2, _) = collision_side_single(normal_side.v2, &vertices_normal);



    // check that the two directions point in the same direction. i.e. the line both points on the normal_side is on the same side of the line between vertices

    let d1x = if d1.x >= 0.0 { 1 } else { -1 };
    let d1y = if d1.y >= 0.0 { 1 } else { -1 };
    let d2x = if d2.x >= 0.0 { 1 } else { -1 };
    let d2y = if d2.y >= 0.0 { 1 } else { -1 };
    let col = d1x * d1y != d2x * d2y;

    // if both collide, i.e full collision use the largest other wise smallest


    if mag_1 < mag_2 {
        (col , correction1, mag_1)
    }
    else {
        (col , correction2, mag_1)
    }
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
    let new_x = if line.y == 0.0 { 0.0 } else { - line.y};
    let w = na::Vector3::new(new_x, line.x, line.z).normalize();

    let b = w.transpose() * v1;

    let proj_1 = projection(&v1, &line).dot(&line);
    let proj_2 = projection(&v2, &line).dot(&line);

    NormalSide{
        w,
        b,
        v1,
        v2
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

    vec! [ v00, v01, v11, v10]

}

pub fn generate_side(vertices: &Vec::<na::Vector3::<f32>>) -> Vec::<Side> {

    let mut res = Vec::new();
    if vertices.len() < 2 {
        println!("empt all over");
        return res;
    }


    let max_idx = vertices.len() - 1;

    for i in 1..=max_idx {
        res.push(Side {
            v1: vertices[i-1],
            v2: vertices[i]
        });
    }

    res.push(Side {
        v1: vertices[max_idx],
        v2: vertices[0]
    });

    res

}

pub fn generate_side_from_bb(b: &CollisionBox) -> Vec::<Side> {
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


    use crate::physics::projection_collision::*;
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



        let (has_col, _) = collision_sat(generate_vertices(&box1), generate_side_from_bb(&box2).as_slice());

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



        let (has_col, _) = collision_sat(generate_vertices(&box1), generate_side_from_bb(&box2).as_slice());

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

        let (has_col, _) = collision_sat(generate_vertices(&box1), generate_side_from_bb(&box2).as_slice());

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
            pos: na::Vector3::new(-8.5, -1.0, 0.0),
            side_len: 1.0,

        };

        let side = generate_normal_side(
            na::Vector3::new(-8.0, 9.0,0.0),
            na::Vector3::new(-8.0, -10.0,0.0),
        );
        let (has_col, dir) = collision_side(generate_vertices(&box1), &side);

        //println!("{}", dir);
        assert!(has_col);
    }



    #[test]
    fn collision_side_right_false() {

        let box1 = CollisionBox {
            pos: na::Vector3::new(9.0, 18.0, 0.0),
            side_len: 1.0,

        };

        let side = generate_normal_side(
            na::Vector3::new(9.0, -10.0,0.0),
            na::Vector3::new(9.0, -9.0,0.0),
        );

        let (has_col, _) = collision_side(generate_vertices(&box1), &side);

        assert!(!has_col);
    }




    #[test]
    fn collision_side_piece_true() {


        let v1 = na::Vector3::new(0.5, 0.5, 0.0);

        let v2 = na::Vector3::new(0.5, -0.5, 0.0);


        let normal_side = generate_normal_side(
            na::Vector3::new(0.0, 0.0,0.0),
            na::Vector3::new(1.0, 0.0,0.0),
        );

        let side = Side {
            v1,
            v2,
        };


        let (has_col, _, _ ) = collision_side_piece(&side, &normal_side);


        assert!(has_col);
    }

    #[test]
    fn collision_side_piece_false() {


        let v1 = na::Vector3::new(0.5, 0.5, 0.0);

        let v2 = na::Vector3::new(10.5, -0.1, 0.0);


        let normal_side = generate_normal_side(
            na::Vector3::new(0.0, 0.0,0.0),
            na::Vector3::new(1.0, 0.0,0.0),
        );

        let side = Side {
            v1,
            v2,
        };


        let (has_col, _, _) = collision_side_piece(&side, &normal_side);


        assert!(!has_col);
    }


}
