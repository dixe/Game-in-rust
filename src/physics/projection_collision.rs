use nalgebra as na;



pub struct CollisionBox {
    pub pos: na::Vector3::<f32>,
    pub side_len: f32,
}



pub fn collision_sat(box1: &CollisionBox, box2: &CollisionBox) -> (bool, na::Vector3::<f32>) {



    let vertecies_box_1 = generate_vertecies(box1);
    let vertecies_box_2 = generate_vertecies(box2);


    let mut has_gap = false;

    let mut smallest_overlap = 10000000000000000000.0;
    let mut smallest_overlap_dir = na::Vector3::new(0.0, 0.0, 0.0);

    'sides: for (v1, v2) in generate_sides(box1) {

        let line = (v1 - v2);

        let wall = na::Vector3::new( -line.y, line.x, line.z).normalize();

        let mut box_1_max = 0.0;
        let mut box_1_min = vertecies_box_1[0].dot(&wall);
        for v in &vertecies_box_1 {
            let proj_dot = projection(v, &wall).dot(&wall);

            box_1_max = f32::max(box_1_max, proj_dot);
            box_1_min = f32::min(box_1_min, proj_dot);
        }

        let mut box_2_max = 0.0;
        let mut box_2_min = vertecies_box_2[0].dot(&wall);
        for v in &vertecies_box_2 {
            let proj_dot = projection(v, &wall).dot(&wall);
            box_2_max = f32::max(box_2_max, proj_dot);
            box_2_min = f32::min(box_2_min, proj_dot);
        }


        let offset = (box1.pos - box2.pos).dot(&wall);

        let overlap = (box_1_min <= box_2_min && box_1_max >= box_2_min) ||
            (box_1_min <= box_2_max && box_1_max >= box_2_max);


        has_gap = !overlap;
        /*println!("({}, {}) ({}, {})   .  {}", box_1_min, box_1_max, box_2_min, box_2_max, has_gap);
        println!("({}, {}) ({}, {})", box_1_min <= box_2_min, box_1_max >= box_2_min, box_1_min <= box_2_max, box_1_max >= box_2_max);
        println!("{}\n\n", line);

        println!("{}", &box1.pos);
         */

        if(has_gap){
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


fn generate_vertecies(b: &CollisionBox) ->Vec::<na::Vector3::<f32>> {
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


fn generate_sides(b: &CollisionBox) -> Vec::<(na::Vector3::<f32>,na::Vector3::<f32>)> {
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

    vec! [
        (v00, v10),
        (v10, v11),
        (v11, v01),
        (v01, v00)

    ]
}


pub fn collision_x_y(box1: &CollisionBox, box2: &CollisionBox) -> (bool, na::Vector3::<f32>) {

    let mut sides_box_1 = vec! [
        (na::Vector3::new(box1.pos.x, box1.pos.y, 0.0), na::Vector3::new(box1.pos.x + box1.side_len ,box1.pos.y, 0.0)),
        (na::Vector3::new(box1.pos.x, box1.pos.y, 0.0), na::Vector3::new(box1.pos.x, box1.pos.y + box1.side_len, 0.0)),
        (na::Vector3::new(box1.pos.x + box1.side_len, box1.pos.y + box1.side_len, 0.0), na::Vector3::new(box1.pos.x + box1.side_len ,box1.pos.y, 0.0)),
        (na::Vector3::new(box1.pos.x + box1.side_len, box1.pos.y + box1.side_len, 0.0), na::Vector3::new(box1.pos.x, box1.pos.y + box1.side_len, 0.0)),
    ];

    let mut vertecies_box_2 = vec! [
        na::Vector3::new(box2.pos.x, box2.pos.y, 0.0),
        na::Vector3::new(box2.pos.x, box2.pos.y, 0.0),
        na::Vector3::new(box2.pos.x + box2.side_len, box2.pos.y + box2.side_len, 0.0),
        na::Vector3::new(box2.pos.x + box2.side_len, box2.pos.y + box2.side_len, 0.0)
    ];


    let mut with_dist: Vec::<(f32, &na::Vector3::<f32>)>  = Vec::new();
    let mut max_mag = 0.0;
    let mut ret_dir = na::Vector3::new(0.0, 0.0, 0.0);

    let mut has_collision = false;
    for (v1, v2) in sides_box_1 {

        let line = v1 - v2;

        for v in &vertecies_box_2 {
            with_dist.push((projection(&v, &line).magnitude(), v));
        }

        with_dist.sort_unstable_by(|(a,_), (b,_)| a.partial_cmp(b).unwrap());

        let s1 = with_dist[0].1;
        let mut s2 = with_dist[1].1;


        let ot_line = na::Vector3::<f32>::new(line.y, line.x, line.z);


        let ot_s1 = na::Vector3::<f32>::new(s1.y, s1.x, s1.z);
        if false &&  s2.dot(&ot_s1) == 0.0 {
            s2 = with_dist[2].1;
            println!("change");
        }

        let (mut inter, dir1, dir2) = pair_intersect(&s1, &s2, &line);

        let ot_line = na::Vector3::<f32>::new(line.y, line.x, line.z);

        // check if we line up on ex x axis, then handle it with checking the directions are all the same
        if dir1.dot(&ot_line) == 0.0 && dir2.dot(&ot_line) == 0.0 {
            let a1 = (v1 - s1).magnitude();
            let a2 = (v1 - s2).magnitude();
            let b1 = (v2 - s1).magnitude();
            let b2 = (v2 - s2).magnitude();

            // if any abs diff is bigger than 1, then not a overlap, but just same axis
            inter = !(a1 + a2 > box1.side_len + box2.side_len || b1 + b2 > box1.side_len + box2.side_len );

            //println!("{} {} {}, {}", a1, a2, b1, b2);
        }
        let m = dir1.magnitude();
        if m > max_mag {
            max_mag = m;
            ret_dir = dir1;
        }

        let m = dir2.magnitude();

        if m > max_mag {
            max_mag = m;
            ret_dir = dir2;
        }

        //println!("{}", ret_dir);


        has_collision |= inter;
    }

    (has_collision, ret_dir)
}

pub fn projection(from: &na::Vector3::<f32>, onto: &na::Vector3::<f32>) -> na::Vector3::<f32>  {
    (from.dot(onto) / onto.dot(onto)) * onto
}



pub fn pair_intersect(vertex1: &na::Vector3::<f32>, vertex2: &na::Vector3::<f32>,  line: &na::Vector3::<f32>) -> ( bool, na::Vector3::<f32>, na::Vector3::<f32>) {

    let proj1 = projection(vertex1, line);
    let proj2 = projection(vertex2, line);

    let dir1 = proj1 - vertex1;
    let dir2 = proj2 - vertex2;

    //println!("proj1 {}", proj1);
    let res = !((dir1.magnitude() * dir2.magnitude()) > 0.0);

    let m1 = dir1.dot(line);
    let m2 = dir1.dot(line);

    let m3 = dir1.dot(&dir2);


    let res = !(m3 > 0.0);

    //println!("{} {}\n{}, {}, {}, {}", dir1, dir2, m1, m2, m3, res);


    (res, dir1, dir2)



}


#[cfg(test)]
mod tests {

    use crate::physics::projection_collision::{CollisionBox,projection, pair_intersect, collision_sat};
    use nalgebra as na;

    #[test]
    fn test_projection_1() {


        let line =  na::Vector3::new(1.0, 0.0, 0.0);

        let vertex1 = na::Vector3::new(1.9, 1.0, 0.0);

        let proj1 = projection(&vertex1, &line);

        assert_eq!(proj1, na::Vector3::new(1.9, 0.0, 0.0));
    }



    #[test]
    fn no_pair_intersect() {


        let line =  na::Vector3::new(1.0, 0.0, 0.0);

        let vertex1 = na::Vector3::new(1.9, 1.0, 0.0);

        let vertex2 = na::Vector3::new(1.2, 1.0, 0.0);

        let (inter, _, _) = pair_intersect(&vertex1, &vertex2, &line);

        assert_eq!(false, inter)
    }

    #[test]
    fn is_pair_intersect() {


        let line =  na::Vector3::new(1.0, 0.0, 0.0);

        let vertex1 = na::Vector3::new(1.9, 1.0, 0.0);

        let vertex2 = na::Vector3::new(1.2, -1.0, 0.0);

        let (inter, _, _) = pair_intersect(&vertex1, &vertex2, &line);

        assert_eq!(true, inter)
    }


    #[test]
    fn is_pair_intersect_2() {


        let line =  na::Vector3::new(1.0, 0.0, 0.0);

        let vertex1 = na::Vector3::new(1.9, 0.0, 0.0);

        let vertex2 = na::Vector3::new(1.9, 1.0, 0.0);

        let (inter, _, _) = pair_intersect(&vertex1, &vertex2, &line);

        assert_eq!(true, inter)
    }

    #[test]
    fn is_pair_intersect_3() {


        let line =  na::Vector3::new(1.0, 0.0, 0.0);

        let vertex1 = na::Vector3::new(1.9, 0.0, 0.0);

        let vertex2 = na::Vector3::new(2.9, 0.0, 0.0);

        let (inter, _, _) = pair_intersect(&vertex1, &vertex2, &line);

        assert_eq!(true, inter)
    }


    #[test]
    fn is_pair_intersect_4() {


        let line =  na::Vector3::new(1.0, 0.0, 0.0);

        let vertex1 = na::Vector3::new(1.9, 0.0, 0.0);

        let vertex2 = na::Vector3::new(2.5, 0.0, 0.0);

        let (inter, _, _) = pair_intersect(&vertex1, &vertex2, &line);

        assert_eq!(true, inter)
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



        let (has_col, dir) = collision_sat(&box1, &box2);

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



        let (has_col, dir) = collision_sat(&box1, &box2);

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

        let (has_col, dir) = collision_sat(&box1, &box2);

        assert!(!has_col);


    }

}
