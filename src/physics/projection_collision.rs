use nalgebra as na;



pub struct CollisionBox {
    pub pos: na::Vector3::<f32>,
    pub side_len: f32,
}



pub fn collision_sat(vertices: Vec::<na::Vector3::<f32>>, sides: Vec::<(na::Vector3::<f32>,na::Vector3::<f32>)>) -> (bool, na::Vector3::<f32>) {

    let vertices_1 = vertices;

    let vertices_2 = vertices_from_sides(&sides);
    let mut has_gap = false;

    let mut smallest_overlap = 10000000000000000000.0;
    let mut smallest_overlap_dir = na::Vector3::new(0.0, 0.0, 0.0);

    'sides: for (v1, v2) in sides {

        let line = (v1 - v2).normalize();

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

fn vertices_from_sides(sides: &Vec::<(na::Vector3::<f32>,na::Vector3::<f32>)>) -> Vec::<na::Vector3::<f32>> {

    let mut r = Vec::<na::Vector3::<f32>>::new();

    for (v,_) in sides {
        r.push(*v);
    }
    r
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


pub fn generate_sides(b: &CollisionBox) -> Vec::<(na::Vector3::<f32>,na::Vector3::<f32>)> {
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


pub fn projection(from: &na::Vector3::<f32>, onto: &na::Vector3::<f32>) -> na::Vector3::<f32>  {
    (from.dot(onto) / onto.dot(onto)) * onto
}


#[cfg(test)]
mod tests {

    use crate::physics::projection_collision::{CollisionBox,projection, collision_sat};
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
