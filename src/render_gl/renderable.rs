use nalgebra as na;

pub trait Renderable {
    fn render(&self, gl: &gl::Gl, projection: na::Matrix4<f32>, view: na::Matrix4<f32>);
}
