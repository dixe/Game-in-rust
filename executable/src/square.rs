use gl;
use failure;
use crate::render_gl::{self, buffer};
use crate::resources::Resources;
use nalgebra as na;


pub struct Square {
    program: render_gl::Program,
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer
}


impl Square {

    pub fn new(res: &Resources,  gl: &gl::Gl) -> Result<Square, failure::Error> {

        let program = render_gl::Program::from_res(gl, res, "shaders/square" ).unwrap();


        let vertices: Vec<f32> = vec![
            // positions
            0.5,  0.5, 0.0,
            0.5, -0.5, 0.0,
            -0.5, -0.5, 0.0,
            -0.5,  0.5, 0.0,
        ];



        let indices: Vec<u32> = vec![
            0,1,3,
            1,2,3];


        let vbo = buffer::ArrayBuffer::new(gl);
        let ebo = buffer::ElementArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);


        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.static_draw_data(&vertices);

            // 3
            ebo.bind();
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW);


            // 4.
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);
        }

        vbo.unbind();
        vao.unbind();

        Ok(Square {
            program,
            vao,
            _vbo: vbo,
            _ebo: ebo,
        })
    }

    pub fn render(&self, gl: &gl::Gl, projection: na::Matrix4<f32>,  view: na::Matrix4<f32>, model: na::Matrix4<f32>,) {
        self.program.set_used();

        self.vao.bind();
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

            // draw
            gl.DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }
    }
}
