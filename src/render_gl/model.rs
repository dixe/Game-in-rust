use gl;
use crate::render_gl::{self, data, buffer};
use nalgebra as na;
use std::io::BufReader;
use stringreader::StringReader;

use crate::resources::Resources;
use obj;



pub struct Model {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    indices_count: i32,
}


impl Model {

    pub fn load_from_path(gl: &gl::Gl, clr: na::Vector3::<f32>, path: &str, res: &Resources) -> Result<Model, failure::Error> {


        let model_content = res.load_string(path).unwrap();
        let mut str_reader = StringReader::new(&model_content);
        let buf= BufReader::new(str_reader);

        let model: obj::Obj = obj::load_obj(buf).unwrap();

        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 9;


        let mut vertices = Vec::<f32>::new();

        let ebo_data: Vec<u16> = model.indices.clone();

        let ebo = buffer::ElementArrayBuffer::new(gl);


        for v in &model.vertices {
            let pos = v.position;
            // position
            vertices.push(pos[0]);
            vertices.push(pos[1]);
            vertices.push(pos[2]);

            // color
            vertices.push(clr.x);
            vertices.push(clr.y);
            vertices.push(clr.z);

            // normal
            let normal = v.normal;
            vertices.push(normal[0]);
            vertices.push(normal[1]);
            vertices.push(normal[2]);

        }

        let indices_count = model.indices.len() as i32;


        let stride = 9;
        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.static_draw_data(&vertices);

            //3
            ebo.bind();
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (ebo_data.len() * std::mem::size_of::<u16>()) as gl::types::GLsizeiptr,
                ebo_data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW);

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


        Ok(Model {
            vao,
            _vbo: vbo,
            _ebo: ebo,
            indices_count,
        })
    }



    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, model: na::Matrix4<f32>,) {
        shader.set_model(gl, model);

        self.vao.bind();
        unsafe {
            gl.DrawElements(
                gl::TRIANGLES,
                self.indices_count,
                gl::UNSIGNED_SHORT,
                0 as *const gl::types::GLvoid
            );
        }
    }
}
