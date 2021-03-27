use gl;
use crate::render_gl::{self, buffer};
use nalgebra as na;
use std::io::BufReader;
use stringreader::StringReader;
use crate::resources::Resources;
use obj;
use tobj;


pub struct Model {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    pub indices_count: i32,
}


impl Model {

    pub fn load_from_path_tobj(gl: &gl::Gl, clr: na::Vector3::<f32>, path: &str, res: &Resources) -> Result<Model, failure::Error> {


        // TODO RECATOR SO WE GET A FULL PATH FORM RESOURCES
        println!("PAHT {}", path);
        let obj_path = path.to_string() + ".obj";
        let mtl_path = path.to_string() + ".mtl";

        let model_content = res.load_string(&obj_path).unwrap();
        let str_reader = StringReader::new(&model_content);
        let mut obj_buf = BufReader::new(str_reader);



        let (models, materials) = tobj::load_obj_buf(&mut obj_buf, true, |p| {
            match p.file_name().unwrap().to_str().unwrap() {
                m => {
                    let mtl_content = res.load_string(&mtl_path).unwrap();
                    let str_mtl_reader = StringReader::new(&mtl_content);
                    let mut mtl_buf = BufReader::new(str_mtl_reader);
                    tobj::load_mtl_buf(&mut mtl_buf)
                }
            }
        })?;


        let mut model = None;
        for loaded_model in &models {
            println!("MODEL NAME: {:#?}", loaded_model.name);

            if loaded_model.name != "Cube" {

                continue;
            }

            println!("MODEL face len: {:#?}", loaded_model.mesh.num_face_indices.len());


            let vbo = buffer::ArrayBuffer::new(gl);
            let vao = buffer::VertexArray::new(gl);

            let mut vertices = Vec::<f32>::new();

            let ebo_data: Vec<u32> = loaded_model.mesh.indices.clone();

            let ebo = buffer::ElementArrayBuffer::new(gl);


            let mut i = 0;
            while i < loaded_model.mesh.positions.len() {
                // x y z
                println!("{}, {}, {}", loaded_model.mesh.positions[i], loaded_model.mesh.positions[i + 1], loaded_model.mesh.positions[i + 2]);
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
            println!("VERT LEN {}", vertices.len());

            println!("INDICES LEN {}", loaded_model.mesh.indices.len() );
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


            model = Some(Model {
                vao,
                _vbo: vbo,
                _ebo: ebo,
                indices_count,
            });
            break;
        }


        match model {
            Some(m) => Ok(m),
            _ => panic!("No model found"), //TODO make this a failure and not a panic
        }

    }

    pub fn load_from_path_obj_rs(gl: &gl::Gl, clr: na::Vector3::<f32>, path: &str, res: &Resources) -> Result<Model, failure::Error> {

        let model_content = res.load_string(path).unwrap();
        let str_reader = StringReader::new(&model_content);
        let buf = BufReader::new(str_reader);

        let model: obj::Obj = obj::load_obj(buf).unwrap();

        println!("Name is {:#?}", model.name);

        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let mut vertices = Vec::<f32>::new();

        let ebo_data: Vec<u32> = model.indices.iter().map(|x| *x as u32).collect::<Vec<u32>>().clone();

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

        println!("VERT ORIG LEN {}", vertices.len());
        println!("INDICES _ORIGN LEN {}", model.indices.len() );
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
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }
    }
}
