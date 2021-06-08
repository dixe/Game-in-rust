use crate::render_gl::mesh::{GltfMesh};

type v3 = na::Vector3::<f32>;


pub fn triangle() -> GltfMesh {

    let name = "triangle_mesh".to_string();

    let pos_data = vec! [ v3::new(0.0, 0.0, 0.0),
                          v3::new(2.0, 0.0, 0.0),
                          v3::new(2.0, 2.0, 0.0), ];

    let normal_data = vec! [ [0.0, 0.0, 1.0],
                              [0.0, 0.0, 1.0],
                              [0.0, 0.0, 1.0], ];

    let indices_data = vec![ 0, 1, 2];

    let tex_data = vec![ [0.0, 0.0], [0.0, 0.0]];

    let vertex_weights = Vec::new();

    GltfMesh {
        name,
        pos_data,
        normal_data,
        indices_data,
        tex_data,
        vertex_weights,
    }
}
