use crate::level;

use crate::cube;

use crate::render_gl;

use nalgebra as na;

pub struct Scene {

    pub border_positions: Vec::<na::Vector3::<f32>>,
    border : cube::Cube,
}




impl Scene {
    pub fn new(level: &level::Level, ctx: &render_gl::context::Context) -> Result<Scene, failure::Error> {

        let mut border_positions = Vec::<na::Vector3::<f32>>::new();

        let pos = na::Vector3::new(0.0,0.0,0.0);
        let clr = na::Vector3::new(0.6, 0.5, 0.4);
        let border = cube::Cube::new(&ctx.res, clr, &ctx.gl)?;


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
            border
        })
    }

    pub fn render(&self, gl: &gl::Gl, projection: na::Matrix4<f32>,  view: na::Matrix4<f32>) {
        for pos in &self.border_positions {

            let translation = na::Matrix4::new_translation(&pos);

            let model_mat = translation *  na::Matrix4::identity();

            self.border.render(gl, projection, view, model_mat);
        }

    }

}
