use failure;
use gl;

use crate::resources::Resources;
use crate::render_gl;


#[derive(Copy, Clone)]
pub struct Shader {
    program: render_gl::Program,

}

impl Shader {

    pub fn new(shader_name: &str, res: &Resources, gl: &gl::Gl) -> Result<Shader, failure::Error> {

        let shader_prefix = "shaders/".to_owned();
        let full_name = shader_prefix + shader_name;
        let program = render_gl::Program::from_res(gl, res, &full_name).unwrap();

        Ok(Shader {
            program
        })
    }

    pub fn set_used(&self, gl: &gl::Gl, projection: na::Matrix4<f32>,  view: na::Matrix4<f32>, model: na::Matrix4<f32>) {
        self.program.set_used();

        unsafe {
            let proj_str = std::ffi::CString::new("projection").unwrap();
            let view_str = std::ffi::CString::new("view").unwrap();
            let model_str = std::ffi::CString::new("model").unwrap();

            let proj_loc = gl.GetUniformLocation(
                self.program.id(),
                proj_str.as_ptr() as *mut gl::types::GLchar);

            let view_loc = gl.GetUniformLocation(
                self.program.id(),
                view_str.as_ptr() as *mut gl::types::GLchar);

            let model_loc = gl.GetUniformLocation(
                self.program.id(),
                model_str.as_ptr() as *mut gl::types::GLchar);


            gl.UniformMatrix4fv(
                proj_loc,
                1,
                gl::FALSE,
                projection.as_slice().as_ptr() as *const f32);
            gl.UniformMatrix4fv(
                view_loc,
                1,
                gl::FALSE,
                view.as_slice().as_ptr() as *const f32);
            gl.UniformMatrix4fv(
                model_loc,
                1,
                gl::FALSE,
                model.as_slice().as_ptr() as *const f32);
        }


    }
}
