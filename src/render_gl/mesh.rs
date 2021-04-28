
use crate::render_gl::{self, buffer};
use gl;


pub struct SkinnedMesh {
    mesh: Mesh,
    pub joint_names: Vec<String>,
    pub inverse_bind_poses: Vec<na::Matrix4<f32>>,
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

impl SkinnedMesh {
    pub fn from_gltf(file_path: &str, gl: &gl::Gl, index_map: &std::collections::HashMap<u16,usize>) -> Result<SkinnedMesh, failure::Error> {

        let (gltf, buffers, _) = gltf::import(file_path)?;

        //println!("{:#?}", gltf);
        //panic!("");


        let mut inter_joint_index: Vec::<u16> = Vec::new();

        for skin in gltf.skins() {
            for node in skin.joints() {
                let index = node.index();
                inter_joint_index.push(index as u16);
            }
        }

        for mesh in gltf.meshes() {
            println!("Meshes");
            println!("{:#?}", mesh.name());
        }

        for mesh in gltf.meshes() {
            println!("Loading Mesh index = {:#?} name = {:#?}", mesh.index(), mesh.name());

            let mut vertex_data = Vec::new();

            let mut normal_data = Vec::new();

            let mut tex_data = Vec::new();

            let mut indices_data = Vec::new();

            let mut joints_data = Vec::new();

            let mut weights_data = Vec::new();


            let set = 0;

            let base_rot = na::UnitQuaternion::from_euler_angles(0.0, 0.0 , -90.0_f32.to_radians()).to_homogeneous();

            for primitive in mesh.primitives() {

                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                if let Some(iter) = reader.read_positions() {
                    for pos in iter {
                        let p = base_rot * na::Vector4::new(pos[0], pos[1], pos[2], 1.0);

                        let p1 = na::Vector3::new(p.x, p.y, p.z);
                        vertex_data.push(p1);
                    }
                }

                if let Some(iter) = reader.read_normals() {
                    for norm in iter {
                        normal_data.push(norm);
                    }
                }

                if let Some(reader) = reader.read_tex_coords(set) {
                    for tex in reader.into_f32() {
                        tex_data.push(tex);
                    }
                }

                if let Some(reader) = reader.read_indices() {
                    for tex in reader.into_u32() {
                        indices_data.push(tex);
                    }
                }


                if let Some(reader) = reader.read_weights(set) {
                    for w in  reader.into_f32() {
                        weights_data.push(w);
                    }
                }


                println!("{:#?}", index_map);
                if let Some(reader) = reader.read_joints(set) {
                    let mut c = 0;
                    for j in reader.into_u16() {
                        let mut data: [usize; 4] = [0; 4];
                        for (i, index) in j.iter().enumerate() {

                            // index is into the skins.joints array, which has a list of node indexes
                            // so we have to map from index into joints to
                            data[i] = match index_map.get(&inter_joint_index[*index as usize]) {
                                Some(mapping) => *mapping,
                                None => {
                                    println!("{}, {:?}\n{:?}", c, j, weights_data[c]);
                                    panic!("Non mapped bone has weights. Check weight paint for {}", *index)
                                }
                            };
                        }

                        c += 1;
                        joints_data.push(data);
                    }
                }
                //panic!("");

            }

            println!("{:#?}\n{:#?}", joints_data.len(), weights_data.len());

            println!(
                "Vertices {:#?}, Normals {:?} tex {:?}, indices {}, joints {}, weights {}",
                vertex_data.len(),
                normal_data.len(),
                tex_data.len(),
                indices_data.len(),
                joints_data.len(),
                weights_data.len(),
            );


            let vertex_weights = reduce_to_2_joints(&joints_data, &weights_data);

            println!("{:#?}", indices_data.len());




            let mesh = load_mesh_gltf(gl, &vertex_data, &normal_data, &indices_data, &tex_data, &vertex_weights);

            return Ok(SkinnedMesh {
                mesh,
                inverse_bind_poses: Vec::<na::Matrix4<f32>>::new(),
                joint_names: Vec::new(),
            });
        }

        panic!("NO MESH LOADED EXITING");
    }



    pub fn render(
        &self,
        gl: &gl::Gl,
        shader: &render_gl::Shader,
        model: na::Matrix4<f32>,
        bones: &[na::Matrix4<f32>],
    ) {
        let bones_str = std::ffi::CString::new("uBones").unwrap();
        let len: i32 = bones.len() as i32;

        unsafe {
            let bones_loc = gl.GetUniformLocation(
                shader.program_id(),
                bones_str.as_ptr() as *mut gl::types::GLchar,
            );

            gl.UniformMatrix4fv(bones_loc, len, gl::FALSE, bones.as_ptr() as *const f32);

            shader.set_model(gl, model);

            self.mesh.vao.bind();

            gl.DrawElements(
                gl::TRIANGLES,
                self.mesh.indices_count,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
        }
    }
}

fn load_mesh_gltf(
    gl: &gl::Gl,
    pos_data: &Vec<na::Vector3::<f32>>,
    norm_data: &Vec<[f32; 3]>,
    ebo_data: &Vec<u32>,
    tex_data: &Vec<[f32; 2]>,
    skinning_data: &Vec<VertexWeights>
) -> Mesh {
    let vbo = buffer::ArrayBuffer::new(gl);
    let vao = buffer::VertexArray::new(gl);

    let mut vertex_data = Vec::<f32>::new();

    let ebo = buffer::ElementArrayBuffer::new(gl);

    let indices_count = ebo_data.len();

    //println!("{:#?}", skinning_data);
    for i in 0..pos_data.len() {
        vertex_data.push(pos_data[i].x);
        vertex_data.push(pos_data[i].y);
        vertex_data.push(pos_data[i].z);

        //NORMAL

        vertex_data.push(norm_data[i][0]);
        vertex_data.push(norm_data[i][1]);
        vertex_data.push(norm_data[i][2]);

        // BONE WEIGHTS

        vertex_data.push(skinning_data[i].weights[0]);
        vertex_data.push(skinning_data[i].weights[1]);

        // BONE INDICES

        vertex_data.push(skinning_data[i].joints[0] as f32);
        vertex_data.push(skinning_data[i].joints[1] as f32);

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
            gl::STATIC_DRAW,
        );

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

fn reduce_to_2_joints(joints_data: &Vec<[usize; 4]>, weights_data: &Vec<[f32; 4]>) -> Vec<VertexWeights> {
    // find the two largest weights and use them also normalize the two weights sum to 1

    let mut res = Vec::new();

    for j_index in 0..joints_data.len() {
        let mut max1 = 0.0;
        let mut max2 = 0.0;
        let mut max1_i = 0;
        let mut max2_i = 0;
        for w_index in 0..4 {

            let w = weights_data[j_index][w_index];

            if w >= max1 {
                max2 = max1;
                max2_i = max1_i;
                max1 = w;
                max1_i = w_index;
            }
            else if w >= max2 {
                max2 = w;
                max2_i = w_index;
            }
        }

        if max1_i == max2_i {
            max2 = 0.0;
        }

        if max2 == 0.0 {
            max2_i = max1_i;
        }


        let sum = max1 + max2;
        let max1 = max1 / sum;
        let max2 = max2 / sum;

        let joint_1 = joints_data[j_index][max1_i];
        let joint_2 = joints_data[j_index][max2_i];


        res.push(VertexWeights {
            joints: [joint_1, joint_2],
            weights: [ max1, max2]
        });
    }

    res
}
