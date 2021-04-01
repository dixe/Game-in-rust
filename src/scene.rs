use crate::level;

use crate::cube;

use crate::render_gl;
use crate::floor;

use crate::physics;


use nalgebra as na;

pub struct Scene {
    border_positions: Vec::<na::Vector3::<f32>>,
    border_sides: Vec::<physics::ConvexCollisionShape>,
    border: cube::Cube,
    floor: floor::Floor,
}


impl Scene {
    pub fn new(_level: &level::Level, ctx: &render_gl::context::Context) -> Result<Scene, failure::Error> {

        let border_positions = Vec::<na::Vector3::<f32>>::new();

        let clr = na::Vector3::new(0.6, 0.5, 0.4);
        //TODO should be model ids
        let border = cube::Cube::new(clr, &ctx.gl);

        let floor_clr = na::Vector3::new(0.50, 0.33, 0.6);

        let floor = floor::Floor::new(floor_clr, &ctx.gl)?;


        // create border sides, by only exposing the sides that a entity can reach
        // this is to not do collision with all 4 sides, when only 1 is needed
        let border_sides = Vec::<physics::ConvexCollisionShape>::new();

        let scene = Scene {
            border_positions,
            border,
            floor,
            border_sides,
        };

        // Set level borders
        //scene.set_borders(level);

        Ok(scene)

    }

    fn set_borders(&mut self, level: &level::Level, ) {


        // right
        self.border_sides.push(create_wall_collision_shape(
            na::Vector3::new(9.0, -10.0, 0.0),
            na::Vector3::new(9.0, 9.0, 0.0))
        );



        // left
        self.border_sides.push(create_wall_collision_shape(
            na::Vector3::new(-9.0, 9.0,0.0),
            na::Vector3::new(-9.0, -10.0,0.0),
        ));



        //top
        self.border_sides.push(create_wall_collision_shape(
            na::Vector3::new(-9.0, 9.0,0.0),
            na::Vector3::new(9.0, 9.0,0.0),
        ));


        // bottom
        self.border_sides.push(create_wall_collision_shape(
            na::Vector3::new(9.0, -9.0,0.0),
            na::Vector3::new(-9.0, -9.0,0.0),
        ));

        for (i, item) in level.level_data.iter().enumerate() {

            if *item != 1  {
                continue;
            }

            let x = ((i as i32) % level.width ) - level.width /2;

            let y = ((i as i32) / level.height) - level.height /2;

            let z = 0.5;
            let border_pos = na::Vector3::new(x as f32 , y as f32, z);

            //println!("{}, {}", x,y);

            self.border_positions.push(border_pos);
        }
    }


    pub fn border_sides(&self) -> &Vec::<physics::ConvexCollisionShape> {
        &self.border_sides
    }



    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader) {

        self.floor.render(gl, shader);

        for pos in &self.border_positions {

            let translation = na::Matrix4::new_translation(&pos);

            let model_mat = translation *  na::Matrix4::identity();

            self.border.render(gl, shader, model_mat);
        }
    }


    pub fn add_box(&mut self, center: na::Vector3<f32>) {

        self.border_sides.push( physics::ConvexCollisionShape {
            v1: center + na::Vector3::new(-0.5, -0.5, 0.0),
            v2: center + na::Vector3::new(-0.5, 0.5, 0.0),
            in_between: vec![ center + na::Vector3::new(0.5, 0.5, 0.0)] ,
            last: center + na::Vector3::new(0.5, -0.5, 0.0),

        });

        self.border_positions.push(center);

    }
}


fn create_wall_collision_shape(v1: na::Vector3::<f32>, v2: na::Vector3::<f32>) -> physics::ConvexCollisionShape {

    let dir1 = v2 - v1;

    let behind = na::Vector3::new( dir1.y, dir1.x, dir1.z);

    let dir2 = behind - dir1;

    let flip = dir2.cross(&dir1).z > 0.0;


    let s = if flip {

        physics::ConvexCollisionShape {
            v1: v2 + na::Vector3::new(-0.5, -0.5, 0.0),
            v2: v1 + na::Vector3::new(-0.5, -0.5, 0.0),
            in_between : vec![],
            last: behind,

        }
    } else {
        physics::ConvexCollisionShape {
            v1: v1 + na::Vector3::new(-0.5, -0.5, 0.0),
            v2: v2 + na::Vector3::new(-0.5, -0.5, 0.0),
            in_between : vec![],
            last: behind,
        }
    };
    s
}
