use gl;
use failure;
use crate::render_gl::{Shader, self, data, buffer};
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


pub struct Cube {
    shader: render_gl::Shader,
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
}


impl Cube {

    pub fn new(shader: render_gl::Shader, clr: na::Vector3<f32>, gl: &gl::Gl) -> Result<Cube, failure::Error> {

        let vertices: Vec<f32> = vec![
            // vertecies             // colors
            // front
            -0.5, -0.5, -0.5,    clr.x, clr.y, clr.z,
            0.5, -0.5, -0.5,     clr.x, clr.y, clr.z,
            0.5, 0.5, -0.5,     clr.x, clr.y, clr.z,
            -0.5, 0.5, -0.5,    clr.x, clr.y, clr.z,

            //back
            -0.5, -0.5, 0.5,    clr.x, clr.y, clr.z,
            0.5, -0.5, 0.5,     clr.x, clr.y, clr.z,
            0.5, 0.5, 0.5,     clr.x, clr.y, clr.z,
            -0.5, 0.5, 0.5,    clr.x, clr.y, clr.z,
        ];


        let ebo_data: Vec<u8> = vec![
            // front
	    0, 1, 2,
	    2, 3, 0,
	    // right
	    1, 5, 6,
	    6, 2, 1,
	    // back
	    7, 6, 5,
	    5, 4, 7,
	    // left
	    4, 0, 3,
	    3, 7, 4,
	    // bottom
	    4, 5, 1,
	    1, 0, 4,
	    // top
	    3, 2, 6,
	    6, 7, 3
        ];

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
                (ebo_data.len() * std::mem::size_of::<u8>()) as gl::types::GLsizeiptr,
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


        Ok(Cube {
            shader,
            vao,
            _vbo: vbo,
            _ebo: ebo
        })
    }

    pub fn render(&self, gl: &gl::Gl, projection: na::Matrix4<f32>,  view: na::Matrix4<f32>, model: na::Matrix4<f32>,) {

        self.shader.set_used(gl, projection, view, model);
        self.vao.bind();


        unsafe {
            // draw
            gl.DrawElements(
                gl::TRIANGLES,
                48,
                gl::UNSIGNED_BYTE,
                0 as *const gl::types::GLvoid
            );
        }

    }
}
