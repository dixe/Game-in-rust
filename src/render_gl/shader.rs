use failure;
use gl;

use crate::resources::Resources;
use crate::render_gl;



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

    pub fn set_used(&self) {
        self.program.set_used();

    }

    pub fn set_vec3(&self, gl: &gl::Gl, name: &str, vec3: na::Vector3<f32>) {
        self.program.set_used();
        let vec_str = std::ffi::CString::new(name).unwrap();

        unsafe {

            let vec_loc = gl.GetUniformLocation(
                self.program.id(),
                vec_str.as_ptr() as *mut gl::types::GLchar);

            // println!("{}_loc: {} ", name, vec_loc);


            gl.Uniform3f(vec_loc, vec3.x, vec3.y, vec3.z);
        }
    }


    pub fn set_projection_and_view(&self, gl: &gl::Gl, projection: na::Matrix4<f32>, view: na::Matrix4<f32>) {
        self.program.set_used();
        unsafe {
            let proj_str = std::ffi::CString::new("projection").unwrap();
            let view_str = std::ffi::CString::new("view").unwrap();

            let proj_loc = gl.GetUniformLocation(
                self.program.id(),
                proj_str.as_ptr() as *mut gl::types::GLchar);

            let view_loc = gl.GetUniformLocation(
                self.program.id(),
                view_str.as_ptr() as *mut gl::types::GLchar);

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
        }
    }

    pub fn set_model(&self, gl: &gl::Gl, model: na::Matrix4<f32>) {

        self.program.set_used();

        unsafe {
            let model_str = std::ffi::CString::new("model").unwrap();

            let model_loc = gl.GetUniformLocation(
                self.program.id(),
                model_str.as_ptr() as *mut gl::types::GLchar);

            gl.UniformMatrix4fv(
                model_loc,
                1,
                gl::FALSE,
                model.as_slice().as_ptr() as *const f32);
        }

    }
}
