use nalgebra as na;
use std::ops::Index;


#[derive(Debug)]
pub struct ConvexCollisionShape {
    pub v1: na::Vector3::<f32>,
    pub v2: na::Vector3::<f32>,
    pub in_between: Vec<na::Vector3::<f32>>,
    pub last: na::Vector3::<f32>,
    pub center: na::Vector3::<f32>,
}





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


fn get_shape_vertices(shape: &ConvexCollisionShape) -> Vec<na::Vector3::<f32>> {

    let mut res = Vec::new();

    res.push(shape.v1);
    res.push(shape.v2);

    for v in &shape.in_between {
        res.push(*v);
    }

    res.push(shape.last);
    res
}

fn generate_shape_sides(shape: &ConvexCollisionShape) -> Vec<Side> {
    let mut res = Vec::new();

    res.push(  Side { v1: shape.v1, v2: shape.v2});

    let mut next_v1 = shape.v2;

    let mut next_v2 = shape.v2;

    for v in &shape.in_between {

        next_v1 = next_v2;

        next_v2 = *v;

        res.push(Side { v1: next_v1, v2: next_v2});
    }

    res.push(  Side { v1: next_v2, v2: shape.last});

    res.push( Side { v1: shape.last, v2: shape.v1});

    res
}



pub fn collision_sat_shapes(shape_1: &ConvexCollisionShape, shape_2 : &ConvexCollisionShape) -> (bool, na::Vector3::<f32>) {

    let verticies_1 = get_shape_vertices(shape_1);

    let edges = generate_shape_sides(shape_2);

    // println!(" shape_1{:#?}", shape_1);

    let (col, dir, mag) = collision_sat(verticies_1, &edges);

    let dir2 = (shape_2.center - shape_1.center).normalize();
    if col {
        println!("FINAL CORRECTION MAG {} \n: {:#?}", mag, dir);
    }

    (col, dir * mag)


}

pub fn collision_sat(vertices: Vec::<na::Vector3::<f32>>, sides: &[Side]) -> (bool,  na::Vector3<f32>,f32) {

    let vertices_1 = vertices;

    let vertices_2 = vertices_from_sides(&sides);
    let mut has_gap = false;

    let mut smallest_overlap = 10000000.0;
    let mut smallest_overlap_dir = na::Vector3::new(0.0, 0.0, 0.0);
    let mut above = false;

    // println!("v1s: {:#?}", vertices_1);
    'sides: for s in sides {

        let line = (s.v2 - s.v1).normalize();

        let wall = na::Vector3::new( - line.y, line.x, line.z).normalize();

        let mut box_1_max = vertices_1[0].dot(&wall);
        let mut box_1_min = vertices_1[0].dot(&wall);
        for v in &vertices_1 {
            let proj_dot = projection(v, &wall).dot(&wall);

            box_1_max = f32::max(box_1_max, proj_dot);
            box_1_min = f32::min(box_1_min, proj_dot);
            //println!("v1: {:#?}", proj_dot);
        }


        let mut box_2_max = s.v1.dot(&wall);
        let mut box_2_min = s.v2.dot(&wall);
        for v in &vertices_2 {
            let proj_dot = projection(v, &wall).dot(&wall);
            box_2_max = f32::max(box_2_max, proj_dot);
            box_2_min = f32::min(box_2_min, proj_dot);
            //println!("v2: {:#?}", proj_dot);
        }



        has_gap = box_1_min >= box_2_max || box_2_min >= box_1_max;

        if has_gap {
            break 'sides;
        }


        println!("DIFF 1 AND DIFF 2 ({}, {})", box_1_max - box_2_min, box_2_max - box_1_min );
        let mut smaller = f32::min(box_1_max - box_2_min, box_2_max - box_1_min);

        if smaller < smallest_overlap {
            above = box_1_max - box_2_min < box_2_max - box_1_min ;

            smallest_overlap = smaller;
            smallest_overlap_dir = wall;
            if above {
                //   println!("It came from above");
            } else {
                // println!("It came from below");
            }
        }

    }

    // 1 to 2
    let mut p1 = vertices_1[0];
    p1.x -= 0.5;
    p1.y -= 0.5;
    let mut p2 = vertices_2[0];
    p2.x -= 0.5;
    p2.y -= 0.5;

    let correct_dir = (p2 - p1).normalize();

    println!("CORRECT_ CALC: {:#?}", correct_dir);

    if !above {
        smallest_overlap *= -1.0;
    }

    (!has_gap, smallest_overlap_dir, smallest_overlap)

}

pub fn collision_sat_2(vertices: Vec::<na::Vector3::<f32>>, sides: &[Side]) -> (bool, na::Vector3::<f32>) {

    let vertices_1 = vertices;

    let vertices_2 = vertices_from_sides(&sides);
    let mut has_gap = false;

    let mut smallest_overlap = 10000000000000000000.0;
    let mut smallest_overlap_dir = na::Vector3::new(0.0, 0.0, 0.0);

    // println!("v1s: {:#?}", vertices_1);
    'sides: for s in sides {

        let line = (s.v2 - s.v1).normalize();

        let wall = na::Vector3::new( - line.y, line.x, line.z).normalize();

        let mut box_1_max = -1000000.0;
        let mut box_1_min = 1000.0;//projection(&vertices_1[0], &wall).dot(&wall);
        for v in &vertices_1 {
            let proj_dot = projection(v, &wall).dot(&wall);

            box_1_max = f32::max(box_1_max, proj_dot);
            box_1_min = f32::min(box_1_min, proj_dot);
            //println!("v1: {:#?}", proj_dot);
        }


        let mut box_2_max = -100000.0;
        let mut box_2_min = 10000.0;//projection(&vertices_2[0], &wall).dot(&wall);
        for v in &vertices_2 {
            let proj_dot = projection(v, &wall).dot(&wall);
            box_2_max = f32::max(box_2_max, proj_dot);
            box_2_min = f32::min(box_2_min, proj_dot);
            //println!("v2: {:#?}", proj_dot);
        }



        has_gap = box_1_min > box_2_max || box_2_min > box_1_max;

        /*
        //println!(" gap side: {:#?}", s);
        println!(" ({}, {}) - ({}, {})", box_1_min, box_1_max, box_2_min, box_2_max);
        println!(" GAP SIZE ({} , {})", box_1_min - box_2_max, box_2_min - box_1_max);
         */
        if has_gap {
            /*
            println!(" gap side: {:#?}", s);
            println!(" ({}, {}) - ({}, {})", box_1_min, box_1_max, box_2_min, box_2_max);
            println!(" ({} - {})", box_1_min >= box_2_max, box_2_min >= box_1_max);
             */

            break 'sides;
        }

        let mut proj_side = projection(&s.v1, &wall).dot(&wall);

        let mut smaller = f32::min(box_1_max - box_2_min, box_2_max - box_1_min);


        let calc = f32::min((proj_side - box_1_max).abs(), (proj_side - box_1_min).abs());

        println!("Smaller = {}, smallest overlap = {}, cals = {}", smaller, smallest_overlap, calc);

        //println!("proj_side, proj_side_1, max, min {}, {} |   |, {}, {}", proj_side, proj_side_2, box_1_max, box_1_min);
        // parallel side of a box creats the same projection wall line for both sides. Only take the one where the side we are looking at
        // gives the smallest distance out

        if smaller <= smallest_overlap {
            // smallest_overlap = f32::min((proj_side - box_1_max).abs(), (proj_side - box_1_min).abs());
            //smallest_overlap_dir = wall;
            //smaller = f32::min((proj_side - box_1_max).abs(), (proj_side - box_1_min).abs());
        }


        // original
        let smaller = f32::min(box_1_max - box_2_min, box_2_max - box_1_min);

        //let smaller = calc;
        //println!(" gap side: {:#?}", s);
        //println!(" WALL NORMAL: {:#?}\nSMALLER: {}", wall*smaller, smaller);
        //println!("SMALLER: {}", smaller);
        if smaller < smallest_overlap && calc != 0.0 {
            smallest_overlap = smaller;
            smallest_overlap_dir = wall;
        }

    }

    /*
    if !has_gap && false {

    println!("V1 {:#?}", &vertices_1);
    println!("DATA START");
    println!(" overlap_dir: {:#?}\n  {}\n {:#?}", smallest_overlap_dir, smallest_overlap, smallest_overlap_dir* smallest_overlap);
    println!("DATA END");
}
     */
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


pub fn generate_collision_shape(b: &CollisionBox) -> ConvexCollisionShape {
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


    ConvexCollisionShape {
        v1: v00,
        v2: v10,
        in_between: vec![v11],
        last: v01,
        center : (v00 + v01 + v10 + v11)/4.0
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


    //(from.dot(onto) / onto.mag() * onto.mag()) *
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



        let (has_col, _, _) = collision_sat(generate_vertices(&box1), generate_side_from_bb(&box2).as_slice());

        assert!(has_col);

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



        let (has_col, _, _) = collision_sat(generate_vertices(&box1), generate_side_from_bb(&box2).as_slice());

        assert!(has_col);

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

        let (has_col, _, _) = collision_sat(generate_vertices(&box1), generate_side_from_bb(&box2).as_slice());

        assert!(!has_col);
    }



    #[test]
    fn sat_shape_true_top_left() {

        let box_ = CollisionBox {
            pos: na::Vector3::new(3.0, 0.0, 0.0),
            side_len: 1.0,
        };

        let player =  CollisionBox {
            pos: na::Vector3::new(3.9, 0.5, 0.0),
            side_len: 1.0,
        };

        let (has_col, dir) = collision_sat_shapes(&generate_collision_shape(&player), &generate_collision_shape(&box_));

        println!("TOP LEFT CORRECTION DIRECTION: {:#?} {}", dir, has_col);
        assert!(dir.x < 0.0);
        assert!(dir.y.abs() < 0.001);
        assert!(has_col);

    }


    #[test]
    fn sat_shape_true_above() {

        let box_ = CollisionBox {
            pos: na::Vector3::new(3.1, 0.0, 0.0),
            side_len: 1.0,
        };

        let player =  CollisionBox {
            pos: na::Vector3::new(3.0, 0.9, 0.0),
            side_len: 1.0,
        };

        let (has_col, dir) = collision_sat_shapes(&generate_collision_shape(&player), &generate_collision_shape(&box_));

        println!("ABOVE CORRECTION DIRECTION: {:#?} {}", dir, has_col);
        assert!(dir.y < 0.0);
        assert!(has_col);

    }

    #[test]
    fn sat_shape_true_below() {

        let box_ = CollisionBox {
            pos: na::Vector3::new(3.0, 0.0, 0.0),
            side_len: 1.0,
        };

        let player =  CollisionBox {
            pos: na::Vector3::new(3.0, -0.9, 0.0),
            side_len: 1.0,
        };

        let (has_col, dir) = collision_sat_shapes(&generate_collision_shape(&player), &generate_collision_shape(&box_));

        println!("BELOW CORRECTION DIRECTION: {:#?} {}", dir, has_col);
        assert!(dir.y > 0.0);
        assert!(has_col);

    }


    #[test]
    fn sat_shape_true_1() {

        let wall = create_wall_collision_shape(
            na::Vector3::new(-9.0, 9.0,0.0),
            na::Vector3::new(9.0, 9.0,0.0));
        let box1 = CollisionBox {
            pos: na::Vector3::new(3.0, 3.0, 0.0),
            side_len: 1.0,
        };

        let (has_col,_) = collision_sat_shapes(&generate_collision_shape(&box1), &wall);

        assert!(!has_col);

    }


    #[test]
    fn sat_shape_false() {

        let wall = create_wall_collision_shape(
            na::Vector3::new(9.0, -10.0, 0.0),
            na::Vector3::new(9.0, 9.0, 0.0));

        let box1 = CollisionBox {
            pos: na::Vector3::new(8.5, 20.0, 0.0),
            side_len: 1.0,
        };

        let (has_col,_) = collision_sat_shapes(&generate_collision_shape(&box1), &wall);

        assert!(!has_col);

    }

    #[test]
    fn sat_shape_false_2() {

        let wall = create_wall_collision_shape(
            na::Vector3::new(-9.0, 9.0,0.0),
            na::Vector3::new(9.0, 9.0,0.0));
        let box1 = CollisionBox {
            pos: na::Vector3::new(3.0, 3.0, 0.0),
            side_len: 1.0,
        };

        let (has_col,_) = collision_sat_shapes(&generate_collision_shape(&box1), &wall);

        assert!(!has_col);

    }


    fn create_wall_collision_shape(v1: na::Vector3::<f32>, v2: na::Vector3::<f32>) -> ConvexCollisionShape {

        let line  = v2 - v1;
        let last = na::Vector3::new( line.y, line.x, line.z);

        let s = ConvexCollisionShape {
            v1: v1,
            v2: v2,
            in_between : vec![],
            last: last,
            center: (v1 + v2 + last)/3.0
        };


        println!("{:#?}",s);

        s
    }
}
