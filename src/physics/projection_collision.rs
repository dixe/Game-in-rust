use nalgebra as na;



#[derive(Debug, Clone)]
pub struct ConvexCollisionShape {
    pub v1: na::Vector3::<f32>,
    pub v2: na::Vector3::<f32>,
    pub in_between: Vec<na::Vector3::<f32>>,
    pub last: na::Vector3::<f32>,
}



impl ConvexCollisionShape {


    pub fn rectangle(bottom_left: &na::Vector3::<f32>, height: f32, width: f32 ) -> ConvexCollisionShape {

        let v00 = *bottom_left ;
        let v01 = *bottom_left +
            na::Vector3::new(
                0.0,
                height,
                0.0);
        let v10 = bottom_left +
            na::Vector3::new(
                width,
                0.0,
                0.0);
        let v11 = bottom_left +
            na::Vector3::new(
                width,
                height,
                0.0);

        ConvexCollisionShape {
            v1: v00,
            v2: v01,
            in_between: vec! [  v11],
            last : v10
        }

    }

}

#[derive(Copy, Clone, Debug)]
pub struct Side {
    pub v1: na::Vector3::<f32>,
    pub v2: na::Vector3::<f32>
}


/*
fn get_entity_vertices(entity: &entity::Physics) -> Vec<na::Vector3::<f32>> {

let mut res = Vec::new();

res.push(entity.collision_shape.v1 + entity.pos);
res.push(entity.collision_shape.v2 + entity.pos);

for v in &entity.collision_shape.in_between {
res.push(*v + entity.pos);
    }

    res.push(entity.collision_shape.last + entity.pos);
    res
}


fn generate_entity_sides(entity: &entity::Physics) -> Vec<Side> {
let mut res = Vec::new();

res.push(  Side { v1: entity.collision_shape.v1 + entity.pos, v2: entity.collision_shape.v2 + entity.pos});

let mut next_v1;

let mut next_v2 = entity.collision_shape.v2 + entity.pos;

for v in &entity.collision_shape.in_between {

next_v1 = next_v2;

next_v2 = *v + entity.pos;

res.push(Side { v1: next_v1, v2: next_v2});
    }

    res.push(  Side { v1: next_v2, v2: entity.collision_shape.last + entity.pos});

    res.push( Side { v1: entity.collision_shape.last + entity.pos, v2: entity.collision_shape.v1 + entity.pos});

    res
}
*/
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

    let mut next_v1;

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


pub fn collision_sat_shapes(shape_1: &ConvexCollisionShape, shape_2: &ConvexCollisionShape) -> (bool, na::Vector3::<f32>) {

    let verticies_1 = get_shape_vertices(shape_1);

    let edges = generate_shape_sides(shape_2);

    let (col, dir, mag) = collision_sat(verticies_1, &edges);

    (col, dir * mag)
}


pub fn collision_sat(vertices: Vec::<na::Vector3::<f32>>, sides: &[Side]) -> (bool,  na::Vector3<f32>,f32) {

    let vertices_1 = vertices;

    let vertices_2 = vertices_from_sides(&sides);
    let mut has_gap = false;

    let mut smallest_overlap = 10000000.0;
    let mut smallest_overlap_dir = na::Vector3::new(0.0, 0.0, 0.0);
    let mut below = false;

    // println!("v1s: {:#?}", vertices_1);
    'sides: for s in sides {

        let line = (s.v2 - s.v1).normalize();

        let wall = na::Vector3::new( - line.y, line.x, line.z).normalize();

        let mut shape_1_max = vertices_1[0].dot(&wall);
        let mut shape_1_min = vertices_1[0].dot(&wall);
        for v in &vertices_1 {
            let proj_dot = projection(v, &wall).dot(&wall);

            shape_1_max = f32::max(shape_1_max, proj_dot);
            shape_1_min = f32::min(shape_1_min, proj_dot);
            //println!("v1: {:#?}", proj_dot);
        }


        let mut shape_2_max = s.v1.dot(&wall);
        let mut shape_2_min = s.v2.dot(&wall);
        for v in &vertices_2 {
            let proj_dot = projection(v, &wall).dot(&wall);
            shape_2_max = f32::max(shape_2_max, proj_dot);
            shape_2_min = f32::min(shape_2_min, proj_dot);
        }

        has_gap = shape_1_min >= shape_2_max || shape_2_min >= shape_1_max;

        if has_gap {
            break 'sides;
        }

        //println!("DIFF 1 AND DIFF 2 ({}, {})", shape_1_max - shape_2_min, shape_2_max - shape_1_min );
        let smaller = f32::min(shape_1_max - shape_2_min, shape_2_max - shape_1_min);

        if smaller < smallest_overlap {
            // if the smallest overlap is from below then we have to reverse the direction.
            // this is a fix for parallel lines in boxes
            below = shape_1_max - shape_2_min > shape_2_max - shape_1_min ;

            smallest_overlap = smaller;
            smallest_overlap_dir = wall;

        }

    }

    if below {
        smallest_overlap *= -1.0;
    }

    (!has_gap, smallest_overlap_dir, smallest_overlap)

}


fn vertices_from_sides(sides: &[Side]) -> Vec::<na::Vector3::<f32>> {

    let mut r = Vec::<na::Vector3::<f32>>::new();

    for s in sides {
        r.push(s.v1);
    }
    r
}

/*
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
     */

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

            let box1 = ConvexCollisionShape::rectangle(
                &na::Vector3::new(1.0, 0.8, 0.0),
                1.0,
                1.0

            );


            let box2 = ConvexCollisionShape::rectangle(
                &na::Vector3::new(1.0, 0.0, 0.0),
                1.0,
                1.0

            );



            let (has_col, _) = collision_sat_shapes(&box1, &box2);

            assert!(has_col);

        }

        #[test]
        fn collision_sat_intersect_2() {

            let box1 = ConvexCollisionShape::rectangle(
                &na::Vector3::new(1.0, 0.0, 0.0),
                1.0,
                1.0

            );


            let box2 = ConvexCollisionShape::rectangle(
                &na::Vector3::new(1.1, 0.0, 0.0),
                1.0,
                1.0

            );



            let (has_col, _) = collision_sat_shapes(&box1, &box2);

            assert!(has_col);

        }

        #[test]
        fn collision_sat_no_intersect() {

            let box1 = ConvexCollisionShape::rectangle(
                &na::Vector3::new(1.0, 0.0, 0.0),
                1.0,
                1.0

            );


            let box2 = ConvexCollisionShape::rectangle(
               &na::Vector3::new(2.1, 0.0, 0.0),
                1.0,
                1.0
            );

            let (has_col, _) = collision_sat_shapes(&box1, &box2);

            assert!(!has_col);
        }



        #[test]
        fn sat_shape_true_top_left() {
            let box_ = ConvexCollisionShape::rectangle(
                &na::Vector3::new(3.0, 0., 0.0),
                1.0,
                1.0
            );

            let player = ConvexCollisionShape::rectangle(
                &na::Vector3::new(3.9, 0.5, 0.0),
                1.0,
                1.0
            );


            let (has_col, dir) = collision_sat_shapes(&player, &box_);

            println!("TOP LEFT CORRECTION DIRECTION: {:#?} {}", dir, has_col);
            assert!(dir.x < 0.0);
            assert!(dir.y.abs() < 0.001);
            assert!(has_col);

        }


        #[test]
        fn sat_shape_true_above() {

            let box_ = ConvexCollisionShape::rectangle(
                &na::Vector3::new(3.1, 0.0, 0.0),
                1.0,
                    1.0
            );

            let shape = ConvexCollisionShape::rectangle(&na::Vector3::new(3.0, 0.9, 0.0), 1.0, 1.0);

            let (has_col, dir) = collision_sat_shapes(&shape, &box_);

            println!("ABOVE CORRECTION DIRECTION: {:#?} {}", dir, has_col);
            assert!(dir.y < 0.0);
            assert!(has_col);

        }

        #[test]
        fn sat_shape_true_below() {

            let box_ = ConvexCollisionShape::rectangle(
                &na::Vector3::new(3.0, 0.0, 0.0),
                1.0,
                1.0
            );

            let shape = ConvexCollisionShape::rectangle(&na::Vector3::new(3.0, -0.9, 0.0), 1.0, 1.0);

            let (has_col, dir) = collision_sat_shapes(&shape, &box_);

            println!("BELOW CORRECTION DIRECTION: {:#?} {}", dir, has_col);
            assert!(dir.y > 0.0);
            assert!(has_col);

        }


        #[test]
        fn sat_shape_true_1() {

            let wall = create_wall_collision_shape(
                na::Vector3::new(-9.0, 9.0,0.0),
                na::Vector3::new(9.0, 9.0,0.0));

            let shape = ConvexCollisionShape::rectangle(&na::Vector3::new(3.0, 3.0, 0.0), 1.0, 1.0);

            let (has_col,dir) = collision_sat_shapes(&shape, &wall);

            assert!(!has_col);

        }


        #[test]
        fn sat_shape_false() {

            let wall = create_wall_collision_shape(
                na::Vector3::new(9.0, -10.0, 0.0),
                na::Vector3::new(9.0, 9.0, 0.0));

            let shape = ConvexCollisionShape::rectangle(&na::Vector3::new(8.5, 20.0, 0.0), 1.0, 1.0);

            let (has_col,_) = collision_sat_shapes(&shape, &wall);

            assert!(!has_col);

        }

        #[test]
        fn sat_shape_false_2() {

            let wall = create_wall_collision_shape(
                na::Vector3::new(-9.0, 9.0,0.0),
                na::Vector3::new(9.0, 9.0,0.0));

            let shape = ConvexCollisionShape::rectangle(&na::Vector3::new(3.0, 3.0, 0.0), 1.0, 1.0);

            let (has_col,_) = collision_sat_shapes(&shape, &wall);

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
            };


            println!("{:#?}",s);

            s
        }
    }
