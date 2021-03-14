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

}


impl Model {

    pub fn load_from_path(gl: &gl::Gl, path: &str, res: &Resources) -> Result<Model, failure::Error> {


        let model_content = res.load_string(path).unwrap();
        let mut str_reader = StringReader::new(&model_content);
        let buf= BufReader::new(str_reader);

        let model: obj::Obj = obj::load_obj(buf).unwrap();

        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 6;


        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.static_draw_data(&model.vertices);

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

            // normals
            gl.VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );

            gl.EnableVertexAttribArray(1);


        }




        Ok(Model {
            vao,
            _vbo: vbo,
        })
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
