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


pub struct Cube {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
}


impl Cube {

    pub fn new(clr: na::Vector3<f32>, gl: &gl::Gl) -> Cube {

        let vertices: Vec<f32> = vec![
            // vertecies             // colors          //normal
            -0.5, -0.5, -0.5,    clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            0.5, -0.5, -0.5,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            0.5,  0.5, -0.5,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            0.5,  0.5, -0.5,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            -0.5,  0.5, -0.5,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            -0.5, -0.5, -0.5,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            -0.5, -0.5,  0.5,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            0.5, -0.5,  0.5,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            0.5,  0.5,  0.5,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            0.5,  0.5,  0.5,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            -0.5,  0.5,  0.5,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            -0.5, -0.5,  0.5,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            -0.5,  0.5,  0.5,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            -0.5,  0.5, -0.5,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            -0.5, -0.5, -0.5,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            -0.5, -0.5, -0.5,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            -0.5, -0.5,  0.5,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            -0.5,  0.5,  0.5,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            0.5,  0.5,  0.5,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            0.5,  0.5, -0.5,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            0.5, -0.5, -0.5,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            0.5, -0.5, -0.5,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            0.5, -0.5,  0.5,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            0.5,  0.5,  0.5,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            -0.5, -0.5, -0.5,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            0.5, -0.5, -0.5,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            0.5, -0.5,  0.5,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            0.5, -0.5,  0.5,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            -0.5, -0.5,  0.5,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            -0.5, -0.5, -0.5,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            -0.5,  0.5, -0.5,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            0.5,  0.5, -0.5,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            0.5,  0.5,  0.5,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            0.5,  0.5,  0.5,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            -0.5,  0.5,  0.5,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            -0.5,  0.5, -0.5,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0
        ];


        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 9;
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

            // colors
            gl.VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );

            gl.EnableVertexAttribArray(1);

            // normals
            gl.VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );

            gl.EnableVertexAttribArray(2);

        }

        vbo.unbind();
        vao.unbind();


        Cube {
            vao,
            _vbo: vbo,
        }
    }

    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, model: na::Matrix4<f32>,) {
        shader.set_model(gl, model);

        self.vao.bind();
        unsafe {

            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                36
            );
        }
    }
}
