
use crate::render_gl::{self, buffer};
use gl;

pub struct SkinnedMesh {
    mesh: Mesh,
    pub inverse_bind_poses: Vec<na::Matrix4<f32>>,
}


pub struct Mesh {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    pub indices_count: i32,
}

impl Mesh {

    pub fn new(gl: &gl::Gl, gltf_mesh: &GltfMesh) -> Self {
        load_mesh_gltf(
            gl,
            &gltf_mesh.pos_data,
            &gltf_mesh.normal_data,
            &gltf_mesh.indices_data,
            &gltf_mesh.tex_data,
            None
        )
    }

    pub fn render(
        &self,
        gl: &gl::Gl,
        shader: &render_gl::Shader,
        model: na::Matrix4<f32>,
    ) {

        unsafe {
            shader.set_model(gl, model);

            self.vao.bind();

            gl.DrawElements(
                gl::TRIANGLES,
                self.indices_count,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct VertexWeights {
    // maybe keep the actual vertex index instead of having it just as the index in the vec this is stored in
    joints: [usize; 2],
    weights: [f32; 2],
}

impl SkinnedMesh {
    pub fn new(gl: &gl::Gl, gltf_mesh: &GltfMesh) -> Self {

        //println!("CREATING SKINNED MESH: {:#?}", gltf_mesh.name);

        let mesh = load_mesh_gltf(
            gl,
            &gltf_mesh.pos_data,
            &gltf_mesh.normal_data,
            &gltf_mesh.indices_data,
            &gltf_mesh.tex_data,
            Some(&gltf_mesh.vertex_weights)
        );

        SkinnedMesh {
            mesh,
            inverse_bind_poses: Vec::<na::Matrix4<f32>>::new(),
        }

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

            self.mesh.render(gl, shader, model);
        }
    }
}



// alternative just load the data. and then we can instanciate it if needed

pub struct GltfMesh {
    name: String,
    pos_data: Vec<na::Vector3::<f32>>,
    normal_data: Vec<[f32; 3]>,
    indices_data: Vec<u32>,
    tex_data: Vec<[f32; 2]>,
    vertex_weights: Vec<VertexWeights>
}

pub struct GltfMeshes {
    pub meshes: std::collections::HashMap::<String, GltfMesh>
}


impl GltfMeshes {

    pub fn hitboxes(&self, base_name: &str) -> Vec::<(String, Vec<na::Vector3::<f32>>)>{

        //println!("HITBOXES FOR {}", base_name);
        let mut res = Vec::new();
        for mesh_data in self.meshes.iter().filter(|kv| kv.0.starts_with(base_name) && kv.0.contains("hitbox")).map(|kv| kv.1) {
            // meshes are triangulated, so we want to detriangulate them before we pass them on.
            // They should have 8 vertices
            // from looking at data it seems liek for at box the layout is v0,v0,v0, v1, v1,v1, v2,v2,v2, v3,v3,v3... v7,v7,v7,
            // So we can just loop over in with step of 2 and thus get the vertices

            let mut hitbox = Vec::new();
            for i in (0..24).step_by(3) {
                hitbox.push(mesh_data.pos_data[i]);
            }
            res.push((mesh_data.name.clone(), hitbox));
        }


        res
    }
}




pub fn meshes_from_gltf(file_path: &str, _gl: &gl::Gl, index_map: &std::collections::HashMap<u16,usize>) -> Result<GltfMeshes, failure::Error> {

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

    let mut res = GltfMeshes {
        meshes: std::collections::HashMap::new()
    };


    for node in gltf.nodes() {
        match node.mesh() {
            Some(m) => {
                //println!("EXTRAS FOR {} {:?}", node.name().unwrap(), node.extras());

                res.meshes.insert(node.name().unwrap().to_string(), load_gltf_mesh_data(&m, &buffers, &index_map, &inter_joint_index)?);
            },
            _ => {}
        };
    }

    println!("Meshes loaded {:#?}", res.meshes.keys());

    Ok(res)

}


fn load_gltf_mesh_data(mesh: &gltf::mesh::Mesh, buffers: &Vec<gltf::buffer::Data>, index_map: &std::collections::HashMap<u16,usize>, inter_joint_index: &Vec::<u16>) -> Result<GltfMesh, failure::Error> {

    let name = mesh.name().unwrap().to_string();

    let mut pos_data = Vec::new();

    let mut normal_data = Vec::new();

    let mut tex_data = Vec::new();

    let mut indices_data = Vec::new();

    let mut joints_data = Vec::new();

    let mut weights_data = Vec::new();

    let set = 0;


    for primitive in mesh.primitives() {

        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        if let Some(iter) = reader.read_positions() {
            for pos in iter {
                let p1 = na::Vector3::new(pos[0], pos[1], pos[2]);
                pos_data.push(p1);
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


    let vertex_weights = reduce_to_2_joints(&joints_data, &weights_data);

    Ok(GltfMesh {
        name,
        pos_data,
        normal_data,
        indices_data,
        tex_data, vertex_weights
    })

}




fn load_mesh_gltf(
    gl: &gl::Gl,
    pos_data: &Vec<na::Vector3::<f32>>,
    norm_data: &Vec<[f32; 3]>,
    ebo_data: &Vec<u32>,
    tex_data: &Vec<[f32; 2]>,
    skinning_data: Option<&Vec<VertexWeights>>
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



        match skinning_data {
            Some(s_data) => {

                // BONE WEIGHTS
                vertex_data.push(s_data[i].weights[0]);
                vertex_data.push(s_data[i].weights[1]);

                // BONE INDICES

                vertex_data.push(s_data[i].joints[0] as f32);
                vertex_data.push(s_data[i].joints[1] as f32);
            },
            _ => {
                // BONE WEIGHTS
                vertex_data.push(0.0);
                vertex_data.push(0.0);

                // BONE INDICES

                vertex_data.push(-1.0);
                vertex_data.push(-1.0);

            }
        }


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
