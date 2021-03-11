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
    #[location = 2]
    normals: data::f32_f32_f32

}


pub struct Floor {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    model_mat: na::Matrix4::<f32>,
}


impl Floor {

    pub fn new(clr: na::Vector3<f32>, gl: &gl::Gl) -> Result<Floor, failure::Error> {



        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (-20.0, -20.0, 0.0).into(),
                clr: (clr.x, clr.y, clr.z, 1.0).into(),
                normals: (0.0, 0.0, 1.0).into(),
            }, // bottom right
            Vertex {
                pos: (-20.0, 20.0, 0.0).into(),
                clr: (clr.x, clr.y, clr.z, 1.0).into(),
                normals: (0.0, 0.0, 1.0).into(),
            },
            Vertex {
                pos: (20.0, 20.0, 0.0).into(),
                clr: (clr.x, clr.y, clr.z, 1.0).into(),
                normals: (0.0, 0.0, 1.0).into(),
            },
            Vertex {
                pos: (20.0, -20.0, 0.0).into(),
                clr: (clr.x, clr.y, clr.z, 1.0).into(),
                normals: (0.0, 0.0, 1.0).into(),
            },
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
                (ebo_data.len() * std::mem::size_of::<u8>()) as gl::types::GLsizeiptr,
                ebo_data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW);


            Vertex::vertex_attrib_pointers(gl);


        }

        vbo.unbind();
        vao.unbind();

        let pos = na::Vector3::<f32>::new(0.0, 0.0, 0.0);

        let model_mat = na::Matrix4::new_translation(&pos);

        Ok(Floor {
            vao,
            _vbo: vbo,
            _ebo: ebo,
            model_mat
        })
    }

    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, projection: na::Matrix4<f32>, view: na::Matrix4<f32>) {
        shader.set_used();

        shader.set_model(gl, self.model_mat);
        self.vao.bind();

        unsafe {

            // draw
            gl.DrawElements(
                gl::TRIANGLES,
                8,
                gl::UNSIGNED_BYTE,
                0 as *const gl::types::GLvoid
            );
        }
    }
}
