use gl;
use crate::render_gl::{self, data, buffer};
use nalgebra as na;



#[derive(VertexAttribPointers,Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
    #[location = 2]
    normal: data::f32_f32_f32,
}


pub struct Line {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
}


impl Line {

    pub fn new(v0: na::Vector3<f32>, v1: na::Vector3<f32>, gl: &gl::Gl) -> Line {

        let vertices: Vec<f32> = vec![v0.x, v0.y, v0.z, v1.x, v1.y, v1.z];

        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 1;
        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.static_draw_data(&vertices);

            // 4.
            // vertecies
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);
        }

        vbo.unbind();
        vao.unbind();

        Line {
            vao,
            _vbo: vbo,
        }
    }

    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, model: na::Matrix4<f32>,) {
        shader.set_model(gl, model);

        self.vao.bind();
        unsafe {
            gl.DrawArrays(
                gl::LINES,
                0,
                2
            );
        }
    }
}
