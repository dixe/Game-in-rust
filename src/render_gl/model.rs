use gl;
use crate::render_gl::{self, buffer};
use nalgebra as na;
use std::io::BufReader;
use stringreader::StringReader;
use crate::resources::Resources;

use tobj;
use crate::entity;


pub struct Model {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    pub indices_count: i32,
}



impl Model {

    pub fn load_from_path_tobj(gl: &gl::Gl, clr: na::Vector3::<f32>, path: &str, res: &Resources) -> Result<(Model, Option<entity::AnchorPoint>), failure::Error> {

        // TODO RECATOR SO WE GET A FULL PATH FORM RESOURCES
        let obj_path = path.to_string();
        let mtl_path = path.replace(".obj", ".mtl");

        let model_content = res.load_string(&obj_path).unwrap();
        let str_reader = StringReader::new(&model_content);
        let mut obj_buf = BufReader::new(str_reader);


        let (models, _materials) = tobj::load_obj_buf(&mut obj_buf, true, |p| {
            match p.file_name().unwrap().to_str().unwrap() {
                _m => {
                    let mtl_content = res.load_string(&mtl_path).unwrap();
                    let str_mtl_reader = StringReader::new(&mtl_content);
                    let mut mtl_buf = BufReader::new(str_mtl_reader);
                    tobj::load_mtl_buf(&mut mtl_buf)
                }
            }
        })?;

        let mut anchor_point = None;
        let mut model = None;


        for loaded_model in &models {
            if loaded_model.name.to_lowercase().starts_with("model") {
                model = Some(load_model(loaded_model, gl, clr));
            }
            else {
                anchor_point = Some(load_anchor(loaded_model));
            }
        }

        match model {
            Some(m) => Ok((m, anchor_point)),
            _ => panic!("No model found"), //TODO make this a failure and not a panic
        }

    }



    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, model: na::Matrix4<f32>,) {
        shader.set_model(gl, model);

        self.vao.bind();
        unsafe {
            gl.DrawElements(
                gl::TRIANGLES,
                self.indices_count,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }
    }
}


fn load_anchor(loaded_model: &tobj::Model) -> entity::AnchorPoint {

    let mut normal = na::Vector3::<f32>::new(0.0, 0.0, 0.0);
    let mut pos = na::Vector3::<f32>::new(0.0, 0.0, 0.0);

    let mut i = 0;
    while i < loaded_model.mesh.positions.len() {
        // x y z
        pos.x += loaded_model.mesh.positions[i];
        pos.y += loaded_model.mesh.positions[i + 1];
        pos.z += loaded_model.mesh.positions[i + 2];


        normal.x += loaded_model.mesh.normals[i];
        normal.y += loaded_model.mesh.normals[i + 1];
        normal.z += loaded_model.mesh.normals[i + 2];

        i += 3;

    }

    pos = pos / ((i/3) as f32);
    normal = normal / ((i/3) as f32);

    entity::AnchorPoint {
        pos,
        normal
    }


}


fn load_model(loaded_model: &tobj::Model, gl: &gl::Gl, clr: na::Vector3::<f32>) -> Model {

    let vbo = buffer::ArrayBuffer::new(gl);
    let vao = buffer::VertexArray::new(gl);

    let mut vertices = Vec::<f32>::new();

    let ebo_data: Vec<u32> = loaded_model.mesh.indices.clone();

    let ebo = buffer::ElementArrayBuffer::new(gl);


    let mut i = 0;
    while i < loaded_model.mesh.positions.len() {
        // x y z
        vertices.push(loaded_model.mesh.positions[i]);
        vertices.push(loaded_model.mesh.positions[i + 1]);
        vertices.push(loaded_model.mesh.positions[i + 2]);

        //COLOR

        vertices.push(clr.x);
        vertices.push(clr.y);
        vertices.push(clr.z);

        //NORMAL

        vertices.push(loaded_model.mesh.normals[i]);
        vertices.push(loaded_model.mesh.normals[i + 1]);
        vertices.push(loaded_model.mesh.normals[i + 2]);


        i += 3;
    }

    let indices_count = loaded_model.mesh.indices.len() as i32;

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


    Model {
        vao,
        _vbo: vbo,
        _ebo: ebo,
        indices_count,
    }

}
