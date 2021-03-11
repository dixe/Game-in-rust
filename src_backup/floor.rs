use gl;
use failure;
use crate::render_gl::{self, data, buffer};
use crate::resources::Resources;
use nalgebra as na;

#[derive(VertexAttribPointers,Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,

}


pub struct Floor {
    program: render_gl::Program,
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    proj_loc: gl::types::GLint,
    view_loc: gl::types::GLint,
    model_loc: gl::types::GLint,
    model_mat: na::Matrix4::<f32>,
}


impl Floor {

    pub fn new(res: &Resources, clr: na::Vector3<f32>, gl: &gl::Gl) -> Result<Floor, failure::Error> {

        let program = render_gl::Program::from_res(gl, res, "shaders/floor" ).unwrap();


        let proj_loc: gl::types::GLint;
        let view_loc: gl::types::GLint;
        let model_loc: gl::types::GLint;

        unsafe{
            let proj_str = std::ffi::CString::new("projection").unwrap();
            let view_str = std::ffi::CString::new("view").unwrap();
            let model_str = std::ffi::CString::new("model").unwrap();

            proj_loc = gl.GetUniformLocation(
                program.id(),
                proj_str.as_ptr() as *mut gl::types::GLchar);

            view_loc = gl.GetUniformLocation(
                program.id(),
                view_str.as_ptr() as *mut gl::types::GLchar);

            model_loc = gl.GetUniformLocation(
                program.id(),
                model_str.as_ptr() as *mut gl::types::GLchar);
        }

        //  set transform matrix
        let vertices: Vec<f32> = vec![
            // positions       // Colors
            20.0,  20.0, 0.0,    clr.x, clr.y, clr.z,
            20.0, -20.0, 0.0,    clr.x, clr.y, clr.z,
            -20.0, -20.0, 0.0,   clr.x, clr.y, clr.z,
            -20.0,  20.0, 0.0,   clr.x, clr.y, clr.z,
        ];



        let ebo_data: Vec<u8> = vec![
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
                (ebo_data.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                ebo_data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW);


            // 4.
            // vertecies
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);


            // colors
            gl.VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(1);
        }

        vbo.unbind();
        vao.unbind();

        let pos = na::Vector3::<f32>::new(0.0, 0.0, 0.0);

        let translation = na::Matrix4::new_translation(&pos);

        let model_mat = translation *  na::Matrix4::identity();

        Ok(Floor {
            program,
            vao,
            _vbo: vbo,
            _ebo: ebo,
            proj_loc,
            view_loc,
            model_loc,
            model_mat
        })
    }

    pub fn render(&self, gl: &gl::Gl, projection: na::Matrix4<f32>, view: na::Matrix4<f32>) {
        self.program.set_used();

        self.vao.bind();

        unsafe {


            gl.UniformMatrix4fv(
                self.proj_loc,
                1,
                gl::FALSE,
                projection.as_slice().as_ptr() as *const f32);
            gl.UniformMatrix4fv(
                self.view_loc,
                1,
                gl::FALSE,
                view.as_slice().as_ptr() as *const f32);
            gl.UniformMatrix4fv(
                self.model_loc,
                1,
                gl::FALSE,
                self.model_mat.as_slice().as_ptr() as *const f32);

            // draw
            gl.DrawElements(
                gl::TRIANGLES,
                12,
                gl::UNSIGNED_BYTE,
                0 as *const gl::types::GLvoid
            );
        }
    }
}
