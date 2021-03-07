use crate::level;

use crate::cube;

use crate::render_gl;
use crate::floor;

use crate::physics;


use nalgebra as na;

pub struct Scene {

    border_positions: Vec::<na::Vector3::<f32>>,
    border_sides: Vec::<physics::NormalSide>,
    border: cube::Cube,
    floor: floor::Floor,
}




impl Scene {
    pub fn new(level: &level::Level, ctx: &render_gl::context::Context) -> Result<Scene, failure::Error> {

        let mut border_positions = Vec::<na::Vector3::<f32>>::new();

        let clr = na::Vector3::new(0.6, 0.5, 0.4);
        //TODO shold be model ids
        let border = cube::Cube::new(&ctx.res, clr, &ctx.gl)?;

        let floor_clr = na::Vector3::new(0.50, 0.33, 0.6);

        let floor =  floor::Floor::new(&ctx.res, floor_clr, &ctx.gl)?;


        // create border sides, by only exposing the sides that a entity can reach
        // this is to not do collision with all 4 sides, when only 1 is needed
        let mut border_sides = Vec::<physics::NormalSide>::new();


        // right

        border_sides.push(physics::generate_normal_side(
            na::Vector3::new(9.0, -10.0, 0.0),
            na::Vector3::new(9.0, 9.0, 0.0),
        ));


        // left
        border_sides.push(physics::generate_normal_side(
            na::Vector3::new(-9.0, 9.0,0.0),
            na::Vector3::new(-9.0, -10.0,0.0),
        ));


        //top
        border_sides.push(physics::generate_normal_side(
            na::Vector3::new(9.0, 9.0,0.0),
            na::Vector3::new(-9.0, 9.0,0.0),
        ));

        // bottom
        border_sides.push(physics::generate_normal_side(
            na::Vector3::new(-9.0, -9.0,0.0),
            na::Vector3::new(9.0, -9.0,0.0),
        ));

        for (i, item) in level.level_data.iter().enumerate() {


            if *item != 1  {
                continue;
            }

            let x = ((i as i32) % level.width ) - level.width /2;

            let y = ((i as i32) / level.height) - level.height /2;

            let border_pos = na::Vector3::new(x as f32 , y as f32, 0.0);

            border_positions.push(border_pos);
        }


        Ok(Scene {
            border_positions,
            border,
            floor,
            border_sides,
        })
    }

    pub fn border_sides(&self) -> &Vec::<physics::NormalSide> {

        &self.border_sides
    }



    pub fn render(&self, gl: &gl::Gl, projection: na::Matrix4<f32>,  view: na::Matrix4<f32>) {

        self.floor.render(gl, projection, view);

        for pos in &self.border_positions {

            let translation = na::Matrix4::new_translation(&pos);

            let model_mat = translation *  na::Matrix4::identity();

            self.border.render(gl, projection, view, model_mat);
        }

    }

}
