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


pub struct Triangle {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray
}


impl Triangle {

    pub fn new(res: &Resources,
               p: na::Vector3<f32>,
               clr: na::Vector3<f32>,
               gl: &gl::Gl
    ) -> Result<Triangle, failure::Error> {

        let program = render_gl::Program::from_res(gl, res, "shaders/triangle" ).unwrap();


        let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (-0.5 + p.x, -0.5 + p.y, 0.0 + p.z).into(),
                clr: (clr.x, clr.y, clr.z, 1.0).into(),
            }, // bottom right
            Vertex {
                pos: (0.5 + p.x , -0.5 + p.y , 0.0 + p.z).into(),
                clr: (clr.x, clr.y, clr.z, 1.0).into(),
            }, // bottom left
            Vertex {
                pos: (0.0 + p.x, 0.5 + p.y, 0.0 + p.z).into(),
                clr: (clr.x, clr.y, clr.z, 1.0).into(),
            } // top
        ];


        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();

        vbo.static_draw_data(&vertices);
        Vertex::vertex_attrib_pointers(gl);

        vbo.unbind();
        vao.unbind();

        Ok(Triangle {
            program,
            _vbo: vbo,
            vao,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );
        }
    }
}
