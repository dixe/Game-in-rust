use gl;
use std::fmt;
use crate::render_gl::{self, buffer};
use collada;


pub struct SkinnedMesh {
    mesh: Mesh,
    pub joint_names: Vec::<String>,
    pub skeleton_name: String,
    pub inverse_bind_poses: Vec::<na::Matrix4::<f32>>
}

pub struct Mesh {
    name: String,
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    pub indices_count: i32,
}


#[derive(Debug, Copy, Clone)]
pub struct VertexWeights {
    // maybe keep the actual vertex index instead of having it just as the index in the vec this is stored in
    joints: [usize; 2],
    weights: [f32; 2],
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

fn vertex_debug_info(obj: &collada::Object, bind_info: &BindInfo) {
    for i in 0..obj.vertices.len() {

        let v = obj.vertices[i];
        let vw = bind_info.vertex_weights[i];

        println!("vertex ({:.3},{:.3},{:.3}), {:?}", v.x, v.y, v.z, vw);

    }

}

impl SkinnedMesh {

    pub fn from_collada(doc: &collada::document::ColladaDocument, gl: &gl::Gl, name: &str) -> SkinnedMesh {
        let obj_set = doc.get_obj_set().unwrap();

        println!("Materials {:#?}", obj_set.material_library);
        println!("Objects {:#?}", obj_set.objects.len());


        // setup textures for mesh



        for obj in &obj_set.objects {
            println!("Object Name = {:#?}", obj.name);
            println!("veritces_count {:#?}", obj.vertices.len());

            let bind_info = load_vertex_weights(doc, obj);



            let mesh = load_mesh(obj, &bind_info.vertex_weights, gl, name.to_string());

            //println!("{:#?}", bind_info.joint_names);
            return SkinnedMesh {
                mesh,
                inverse_bind_poses: bind_info.inverse_bind_poses,
                joint_names: bind_info.joint_names.clone(),
                skeleton_name: bind_info.skeleton_name,
            };
        }


        panic!("No models what do to \n\n\n\n");
    }


    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, model: na::Matrix4<f32>, bones: &[na::Matrix4::<f32>]) {

        let bones_str = std::ffi::CString::new("uBones").unwrap();
        let len: i32 = bones.len() as i32;

        unsafe {
            let bones_loc = gl.GetUniformLocation(
                shader.program_id(),
                bones_str.as_ptr() as *mut gl::types::GLchar);


            gl.UniformMatrix4fv(
                bones_loc,
                len,
                gl::FALSE,
                bones.as_ptr() as *const f32);


            shader.set_model(gl, model);

            self.mesh.vao.bind();

            gl.DrawElements(
                gl::TRIANGLES,
                self.mesh.indices_count,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }
    }
}


fn load_mesh(obj: &collada::Object, vert_joints: &Vec::<VertexWeights>, gl: &gl::Gl, name: String) -> Mesh {

    let vbo = buffer::ArrayBuffer::new(gl);
    let vao = buffer::VertexArray::new(gl);

    let mut vertex_data = Vec::<f32>::new();

    let mut ebo_data = Vec::<u32>::new();

    let ebo = buffer::ElementArrayBuffer::new(gl);

    let mut verts = Vec::new();
    let mut vert_norms = Vec::new();
    let mut vert_tex = Vec::new();

    let mut vert_weights = Vec::new();

    // Fill EBO and vertex normals
    for geo in &obj.geometry {

        println!("geo_count {:#?}", obj.geometry.len());

        println!("smooth shading {:#?}", geo.smooth_shading_group);
        for mesh in &geo.mesh {

            println!("mesh_count {:#?}", geo.mesh.len());


            match mesh {
                collada::PrimitiveElement::Triangles(triangles) => {
                    let tri_norms = triangles.normals.as_ref().unwrap();
                    let tri_tex = triangles.tex_vertices.as_ref().unwrap();
                    println!("obj normals {:#?}", tri_norms.len());

                    for i in 0..triangles.vertices.len() {
                        let (v_0, v_1, v_2) = triangles.vertices[i];
                        let (n_0, n_1, n_2) = tri_norms[i];
                        let (t_0, t_1, t_2) = tri_tex[i];

                        if vert_joints.len() > 0 {
                            vert_weights.push(vert_joints[v_0]);
                            vert_weights.push(vert_joints[v_1]);
                            vert_weights.push(vert_joints[v_2]);
                        }
                        else {
                            let vw = VertexWeights {
                                joints: [0, 0],
                                weights: [ 1.0, 0.0]
                            };
                            vert_weights.push(vw);
                            vert_weights.push(vw);
                            vert_weights.push(vw);
                        }

                        ebo_data.push((i * 3) as u32);
                        ebo_data.push((i * 3 + 1) as u32);
                        ebo_data.push((i * 3 + 2) as u32);

                        verts.push(obj.vertices[v_0]);
                        verts.push(obj.vertices[v_1]);
                        verts.push(obj.vertices[v_2]);

                        vert_norms.push(obj.normals[n_0]);
                        vert_norms.push(obj.normals[n_1]);
                        vert_norms.push(obj.normals[n_2]);

                        vert_tex.push(obj.tex_vertices[t_0]);
                        vert_tex.push(obj.tex_vertices[t_1]);
                        vert_tex.push(obj.tex_vertices[t_2]);

                    }

                    println!("triVerts: {:#?}", triangles.vertices.len());
                },

                _=> {
                    panic!("Not triangles");
                }
            };
        }
    }



    let indices_count = verts.len();

    for i in 0..verts.len() {
        // x y z

        let vert = verts[i];

        let norm = vert_norms[i];

        let tex = vert_tex[i];

        //println!("vertex ({:.3},{:.3},{:.3}), TEX {:.03},{:0.3}", vert.x, vert.y, vert.z, tex.x, tex.y);

        vertex_data.push(vert.x as f32);
        vertex_data.push(vert.y as f32);
        vertex_data.push(vert.z as f32);

        //NORMAL
        vertex_data.push(norm.x as f32);
        vertex_data.push(norm.y as f32);
        vertex_data.push(norm.z as f32);


        // BONE WEIGHTS

        let joint_weights = vert_weights[i];

        vertex_data.push(joint_weights.weights[0]);
        vertex_data.push(joint_weights.weights[1]);


        // BONE INDICES
        vertex_data.push(joint_weights.joints[0] as f32);
        vertex_data.push(joint_weights.joints[1] as f32);


        // TEXTURE INFO

        vertex_data.push(tex.x as f32);
        vertex_data.push(tex.y as f32);


    }

    let stride = ((3 + 3 + 2 + 2 + 2) * std::mem::size_of::<f32>()) as gl::types::GLint;
    unsafe {
        // 1
        vao.bind();

        // 2.
        vbo.bind();
        vbo.static_draw_data(&vertex_data);

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
            stride,
            0 as *const gl::types::GLvoid,
        );
        gl.EnableVertexAttribArray(0);


        // normals
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );

        gl.EnableVertexAttribArray(1);


        // bone weights

        gl.VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );

        gl.EnableVertexAttribArray(2);


        // bone indices
        gl.VertexAttribPointer(
            3,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (8 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl.EnableVertexAttribArray(3);


        // bone indices
        gl.VertexAttribPointer(
            4,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (10 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );

        gl.EnableVertexAttribArray(4);




    }

    println!("indices count {:#?}", indices_count);

    let mesh = Mesh {
        vao,
        _vbo: vbo,
        _ebo: ebo,
        indices_count: indices_count as i32,
        name
    };

    mesh
}

struct VWeights {
    joints: Vec::<usize>,
    weights: Vec::<f32>
}


struct BindInfo {
    vertex_weights: Vec::<VertexWeights>,
    joint_names: Vec::<String>,
    skeleton_name: String,
    inverse_bind_poses: Vec<na::Matrix4::<f32>>,
}



fn load_vertex_weights(doc: &collada::document::ColladaDocument, obj: &collada::Object) -> BindInfo {
    let mut vert_joints = Vec::<VWeights>::new();

    let bind_data = doc.get_bind_data_set().unwrap();



    for i in 0..obj.vertices.len() {
        vert_joints.push(VWeights {
            joints: Vec::new(),
            weights: Vec::new(),
        });
    }


    let mut joint_names = Vec::new();
    let mut skeleton_name: String = "".to_string();
    let mut inverse_bind_poses = Vec::new();

    for bind in &bind_data.bind_data {

        inverse_bind_poses = bind.inverse_bind_poses.iter().map(|mat| map_mat4(mat)).collect();

        skeleton_name = bind.skeleton_name.as_ref().unwrap().clone();
        for vw in &bind.vertex_weights {
            vert_joints[vw.vertex].joints.push(vw.joint.into());
            vert_joints[vw.vertex].weights.push(bind.weights[vw.weight]);

        }

        joint_names = bind.joint_names.clone();
    }


    let mut res =  Vec::<VertexWeights>::new();
    let mut index = 0;
    for vertex in &vert_joints {

        match vertex.joints.len() {
            0 => {
                println!("ZERO VERT JOINTSS");

            },
            1 => {
                res.push(VertexWeights {
                    joints: [vertex.joints[0], vertex.joints[0]],
                    weights: [ vertex.weights[0], 0.0]
                });
            },
            2 => {
                res.push(VertexWeights {
                    joints: [vertex.joints[0], vertex.joints[1]],
                    weights: [ vertex.weights[0], vertex.weights[1]]
                });
            },
            n => {

                // find the two largest weights and use them also normalize the two weights to
                // sum to 1

                let mut max1 = 0.0;
                let mut max2 = 0.0;
                let mut max1_i = 0;
                let mut max2_i = 0;

                for (i,w) in vertex.weights.iter().enumerate() {
                    if *w >= max1 {
                        max2 = max1;
                        max2_i = max1_i;
                        max1 = *w;
                        max1_i = i;
                    }
                    else if *w >= max2 {
                        max2 = *w;
                        max2_i = i;
                    }
                }

                if max1_i == max2_i {
                    max2 = 0.0;
                }
                let sum = max1 + max2;
                let max1 = max1 / sum;
                let max2 = max2 / sum;


                let joint_1 = vertex.joints[max1_i];
                let joint_2 = vertex.joints[max2_i];

                //println!("vertex_id = {} selected = ({}, {}) - ({}, {})\n{:#?}, {:#?}", index, joint_1, joint_2, max1, max2, vertex.joints, vertex.weights );

                index += 1;
                res.push(VertexWeights {
                    joints: [joint_1, joint_2],
                    weights: [ max1, max2]
                });
            }
        };
    }

    BindInfo {
        vertex_weights: res,
        joint_names,
        inverse_bind_poses,
        skeleton_name,
    }
}


fn map_mat4(col_mat: &collada::Matrix4<f32>) -> na::Matrix4::<f32> {

    let mut res = na::Matrix4::<f32>::identity();

    let mut index = 0;

    for i in 0..4 {
        for j in 0..4 {
            res[j*4 + i] =col_mat[i][j];
        }
    }
    res
}
