use crate::render_gl::mesh::{GltfMesh};
use noise::{NoiseFn, Perlin, Seedable};
use perlin_noise::PerlinNoise;

type v3 = na::Vector3::<f32>;

type v2 = na::Vector2::<f32>;


pub fn triangle() -> GltfMesh {

    let name = "triangle_mesh".to_string();

    let pos_data = vec! [ v3::new(0.0, 0.0, 0.0),
                          v3::new(2.0, 0.0, 0.0),
                          v3::new(2.0, 2.0, 0.0),
                          v3::new(0.0, 2.0, 0.0),];

    let normal_data = vec! [ [0.0, 0.0, 1.0],
                              [0.0, 0.0, 1.0],
                              [0.0, 0.0, 1.0],
                              [0.0, 0.0, 1.0], ];

    let indices_data = vec![ 0, 1, 2, 1, 2, 3];

    let tex_index_x = 0.8;
    let tex_index_y = 0.0;

    let tex_data = vec! [ [tex_index_x, tex_index_y],
                           [tex_index_x, tex_index_y],
                           [tex_index_x, tex_index_y],
                           [tex_index_x, tex_index_y],];

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


pub fn perlin_field() -> GltfMesh {



    let mut pos_data = Vec::new();

    let h = 500;
    let w = 500;


    //let mut perlin = PerlinNoise::new();
    let mut perlin = Perlin::new();

    let scale = 20.3;
    //perlin.set_seed(42);

    // set pos data
    for i in 0..h {
        for j in 0..w {
            let i_f = (i as f64) / scale;
            let j_f = (j as f64) / scale;
            let noise: f64  = perlin.get([i_f, j_f]);

            let x = (i as f32);// - ((h/2) as f32);
            let y = (j as f32); // - ((w/2) as f32);

            pos_data.push(v3::new(x, y, (noise * 5.0) as f32));
        }
    }




    let indices_data = indices_for_grid(h, w);
    let normal_data = normals_for_grid(&pos_data, &indices_data, h, w);
    let tex_data = tex_coord_for_grid(&pos_data, h, w);


    GltfMesh {
        name: "perlin_noise_mesh".to_string(),
        pos_data,
        normal_data,
        indices_data,
        tex_data,
        vertex_weights: Vec::new()
    }

}

fn randomGradient(ix: u32, iy: u32) -> v2 {
    let x = ix as f32;
    let y = iy as f32;
    // Random float. No precomputed gradients mean this works for any number of grid coordinates
    let random = 2920.0 * f32::sin(x * 21942.0 + y * 171324.0 + 8912.0) * f32::cos(x * 23157.0 * y * 217832.0 + 9758.0);

    v2::new(f32::cos(random), f32::sin(random))
}

fn normals_for_grid(pos_data: &Vec::<v3>, indices: &Vec::<u32>, h: u32, w: u32) -> Vec::<[f32; 3]> {
    let mut normal_data_vec = Vec::new();

    for i in 0..h {
        for j in 0..w {
            normal_data_vec.push(v3::new(0.0, 0.0, 0.0));
        }
    }

    for i in (0..indices.len()).step_by(3) {


        let i0 = indices[i as usize] as usize;
        let i1 = indices[(i + 1) as usize] as usize;
        let i2 = indices[(i + 2) as usize] as usize;


        let v0 = pos_data[i0];

        let v1 = pos_data[i1];
        let v2 = pos_data[i2];

        let e1 = v1 - v0;
        let e2 = v2 - v1;

        let cross = e1.cross(&e2);
        normal_data_vec[i0] += cross;
        normal_data_vec[i1] += cross;
        normal_data_vec[i2] += cross;
    }

    let mut normal_data = Vec::new();
    for normal in &normal_data_vec {
        let normalized = normal.normalize();
        normal_data.push([normalized.x, normalized.y, normalized.z])
    }
    normal_data
}

fn indices_for_grid(h: u32, w: u32) -> Vec::<u32> {
    let mut indices = Vec::new();
    for i in 0..(h-1) {
        for j in 0..(w-1) {
            // first triangle
            indices.push(to_index(i,j,w) );
            indices.push(to_index(i+1,j,w));
            indices.push(to_index(i,j+1,w));

            // first triangle
            indices.push(to_index(i+1,j,w));
            indices.push(to_index(i+1,j+1,w));
            indices.push(to_index(i,j+1,w));
        }
    }

    indices
}


fn tex_coord_for_grid(pos_data: &Vec::<v3>, h: u32, w: u32) -> Vec::<[f32; 2]> {
    let mut tex_data = Vec::new();
    for i in 0..h {
        for j in 0..w {
            tex_data.push([0.7, 0.2]);
        }
    }

    tex_data
}


fn to_index(i: u32, j: u32, w: u32) -> u32 {
    i * w + j
}
