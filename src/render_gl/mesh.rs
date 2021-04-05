use gl;
use std::fmt;
use crate::render_gl::{self, buffer};
use collada;

pub struct Mesh {
    name: String,
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    pub indices_count: i32,
}


#[derive(Debug)]
struct VertexWeights {

    joints: Vec<usize>,
    weights: Vec<f32>
}

impl fmt::Display for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mesh: {}", self.name)?;
        Ok(())
    }
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mesh: {}", self.name)?;
        Ok(())
    }
}


impl Mesh {

    pub fn from_collada(doc: &collada::document::ColladaDocument, gl: &gl::Gl, name: &str) -> Mesh {

        let bind_data = doc.get_bind_data_set().unwrap();
        let obj_set = doc.get_obj_set().unwrap();

        println!("Objects {:#?}", obj_set.objects.len());

        for obj in &obj_set.objects {
            println!("Object Name = {:#?}", obj.name);
            println!("veritces_count {:#?}", obj.vertices.len());

            let mut vert_joints = Vec::<VertexWeights>::new();

            for i in 0..obj.vertices.len() {
                vert_joints.push(VertexWeights {
                    joints: Vec::new(),
                    weights: Vec::new(),
                });
            }

            for bind in &bind_data.bind_data {
                for vw in &bind.vertex_weights {
                    vert_joints[vw.vertex].joints.push(vw.joint.into());
                    vert_joints[vw.vertex].weights.push(bind.weights[vw.weight]);

                }

                //println!("VertWeights {:#?}", bind.vertex_weights);

            }
            return load_model(obj, gl, name.to_string());
        }


        panic!("No models what do to \n\n\n\n");
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


fn load_model(obj: &collada::Object, gl: &gl::Gl, name: String) -> Mesh {

    let vbo = buffer::ArrayBuffer::new(gl);
    let vao = buffer::VertexArray::new(gl);

    let mut vertices = Vec::<f32>::new();

    let mut ebo_data = Vec::<u32>::new();

    let ebo = buffer::ElementArrayBuffer::new(gl);


    let mut vertex_normals = Vec::<collada::Vertex>::with_capacity(obj.vertices.len());

    for i in 0..obj.vertices.len() {
        vertex_normals.push(collada::Vertex { x: 0.0, y: 0.0, z: 0.0});
    }

    let mut indices_count = 0;

    // Fill EBO and vertex normals
    for geo in &obj.geometry {

        println!("geo_count {:#?}", obj.geometry.len());

        for mesh in &geo.mesh {
            println!("mesh_count {:#?}", geo.mesh.len());
            match mesh {
                collada::PrimitiveElement::Triangles(triangles) => {

                    indices_count = triangles.vertices.len() * 3;
                    let tri_norms = triangles.normals.as_ref().unwrap();
                    for i in 0..triangles.vertices.len() {
                        let (v_0,v_1,v_2) = triangles.vertices[i];
                        ebo_data.push(v_0 as u32);
                        ebo_data.push(v_1 as u32);
                        ebo_data.push(v_2 as u32);

                        let (n_0, n_1, n_2) = tri_norms[i];

                        vertex_normals[v_0] = obj.normals[n_0];
                        vertex_normals[v_1] = obj.normals[n_1];
                        vertex_normals[v_2] = obj.normals[n_2];
                    }
                    println!("triVerts: {:#?}", triangles.vertices.len());
                },

                _=> {
                    panic!("Not triangles");
                }
            };

        }
    }


    for i in 0..obj.vertices.len() {
        // x y z
        let vert = obj.vertices[i];
        let norm = vertex_normals[i];

        vertices.push(vert.x as f32);
        vertices.push(vert.y as f32);
        vertices.push(vert.z as f32);

        //NORMAL
        vertices.push(norm.x as f32);
        vertices.push(norm.y as f32);
        vertices.push(norm.z as f32);
    }


    let stride = 6;
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

    println!("indices count {:#?}", indices_count);

    Mesh {
        vao,
        _vbo: vbo,
        _ebo: ebo,
        indices_count: indices_count as i32,
        name
    }
}
