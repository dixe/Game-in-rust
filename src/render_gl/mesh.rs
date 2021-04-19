use crate::render_gl::{Transformation};
use gl;
use std::fmt;
use crate::render_gl::{self, buffer};


pub struct SkinnedMesh {
    name: String,
    mesh: Mesh,
    pub joint_names: Vec::<String>,
    pub skeleton_name: String,
    pub inverse_bind_poses: Vec::<na::Matrix4::<f32>>
}

pub struct Mesh {
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


/*
fn vertex_debug_info(obj: &collada::Object, bind_info: &BindInfo) {
for i in 0..obj.vertices.len() {

let v = obj.vertices[i];
let vw = bind_info.vertex_weights[i];

println!("vertex ({:.3},{:.3},{:.3}), {:?}", v.x, v.y, v.z, vw);

    }
}

*/

impl SkinnedMesh {


    pub fn from_gltf(gl: &gl::Gl) -> Result<SkinnedMesh, failure::Error> {
        let (gltf, buffers, _) = gltf::import("E:/repos/Game-in-rust/blender_models/player_05.glb")?;


        for mesh in gltf.meshes() {
            println!("Mesh #{}", mesh.index());

            let mut vertex_data = Vec::new();

            let mut normal_data = Vec::new();

            let mut tex_data = Vec::new();

            let mut indices_data = Vec::new();


            for primitive in mesh.primitives() {
                println!("- Primitive #{}", primitive.index());
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));


                if let Some(iter) = reader.read_positions() {
                    for pos in iter {
                        vertex_data.push(pos);
                    }
                }

                if let Some(iter) = reader.read_normals() {
                    for norm in iter {
                        normal_data.push(norm);
                    }
                }

                if let Some(reader) = reader.read_tex_coords(0) {
                    for tex in reader.into_f32() {
                        tex_data.push(tex);
                    }
                }

                if let Some(reader) = reader.read_indices() {
                    for tex in reader.into_u32() {
                        indices_data.push(tex);
                    }
                }


                if let Some(read_joints) = reader.read_joints(0) {
                    //println!("{:#?}", read_joints.into_u16());
                }


                // Find the skins, fx Armature


            }


            println!("Vertices {:#?}, Normals {:?} tex {:?}, indices {}", vertex_data.len(), normal_data.len(), tex_data.len(), indices_data.len());

            let mesh = load_mesh_gltf(gl, &vertex_data, &normal_data, &indices_data, &tex_data);

            return Ok(SkinnedMesh {
                mesh,
                name: "test".to_string(),
                inverse_bind_poses: Vec::<na::Matrix4::<f32>>::new(),
                joint_names: Vec::new(),
                skeleton_name: "tmp".to_string()
            });

        }

        panic!("NO MESH LOADED EXITING");
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


fn load_mesh_gltf(gl: &gl::Gl, pos_data: &Vec::<[f32; 3]>, norm_data: &Vec::<[f32; 3]>, ebo_data: &Vec::<u32>, tex_data: &Vec::<[f32; 2]>) -> Mesh {

    let vbo = buffer::ArrayBuffer::new(gl);
    let vao = buffer::VertexArray::new(gl);

    let mut vertex_data = Vec::<f32>::new();


    let ebo = buffer::ElementArrayBuffer::new(gl);


    let indices_count = ebo_data.len();

    for i in 0..pos_data.len() {

        vertex_data.push(pos_data[i][0]);
        vertex_data.push(pos_data[i][1]);
        vertex_data.push(pos_data[i][2]);

        //NORMAL

        vertex_data.push(norm_data[i][0]);
        vertex_data.push(norm_data[i][1]);
        vertex_data.push(norm_data[i][2]);

        // BONE WEIGHTS

        vertex_data.push(0.0);
        vertex_data.push(0.0);


        // BONE INDICES
        vertex_data.push(0.0);
        vertex_data.push(0.0);

        //vertex_data.push(joint_weights.joints[0] as f32);
        //vertex_data.push(joint_weights.joints[1] as f32);


        // TEXTURE INFO

        vertex_data.push(tex_data[i][0]);
        vertex_data.push(tex_data[i][1]);
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

    let mesh = Mesh {
        vao,
        _vbo: vbo,
        _ebo: ebo,
        indices_count: indices_count as i32,
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


/*
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

 */
