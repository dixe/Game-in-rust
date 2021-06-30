use crate::render_gl::mesh::{GltfMesh};
use noise::{NoiseFn, Perlin, Seedable};

use crate::types::*;
use rand::Rng;

pub fn perlin_field() -> GltfMesh {

    let mut noise_data = Vec::new();

    let h = 100;
    let w = 100;


    let mut perlin = Perlin::new();

    let mut rng = rand::thread_rng();

    let random = rng.gen::<u32>();

    perlin = perlin.set_seed(random);
    println!("{:?}", perlin.seed());

    let scale_x = 7.0;
    let scale_y = 7.0;


    // set pos data

    let mut min = 0.0;
    for i in 0..h {
        for j in 0..w {
            let i_f = ((i as f64) / h as f64) * scale_y;
            let j_f = ((j as f64) / w as f64) * scale_x;
            let noise: f64  = perlin.get([i_f, j_f]);

            min = f64::min(min, noise);
            noise_data.push(V3::new(i as f32, j as f32, noise as f32));
        }
    }

    println!("random = {} seed = {} - min={:?}", random, perlin.seed(), min);

    save_noise_to_image(&noise_data, h, w);

    let indices_data = indices_for_grid(h, w);


    let pos_data = pos_for_grid(&noise_data, h as f32, w as f32);
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

fn save_noise_to_image(pos_data: &Vec::<V3>, h: u32, w: u32) -> Result<(), image::ImageError> {

    let new_scale = 256.0 / 2.0;

    let mut imgbuf = image::ImageBuffer::new(w, h);


    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        let index = to_index(x,y, w) as usize;
        // + scale to make min 0
        // Now max value is scale * 2


        let r = ((pos_data[index].z) * new_scale) as u8;

        *pixel = image::Rgb([r, r, r]);
    }

    imgbuf.save("E:/repos/Game-in-rust/noise_image.png").unwrap();

    Ok(())



}

fn pos_for_grid(noise_data: &Vec::<V3>, h: f32, w: f32) -> Vec::<V3> {

    let mut pos_data = Vec::new();
    for sample in noise_data {
        pos_data.push(V3::new( sample.x - (w/2.0), sample.y - (h/2.0), sample.z * 3.0));
    }

    pos_data

}




fn normals_for_grid(pos_data: &Vec::<V3>, indices: &Vec::<u32>, h: u32, w: u32) -> Vec::<[f32; 3]> {
    let mut normal_data_vec = Vec::new();

    for _i in 0..h {
        for _j in 0..w {
            normal_data_vec.push(V3::new(0.0, 0.0, 0.0));
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



fn tex_coord_for_grid(pos_data: &Vec::<V3>, h: u32, w: u32) -> Vec::<[f32; 2]> {
    let mut tex_data = Vec::new();


    let step_size = 1.0/8.0;
    let offset = 0.04;
    let blue = V2::new(offset, step_size + offset);
    let green = V2::new(step_size * 4.0 + offset, step_size + offset);
    let white = V2::new(offset, step_size * 7.0  + offset);

    let _bright = V2::new(0.0, 0.0);
    let light = V2::new(0.1, 0.0);
    let dark = V2::new(0.2, 0.0);
    let darker = V2::new(0.3, 0.0);

    let mut tex_coords = std::collections::HashSet::new();

    for i in 0..h {
        for j in 0..w {

            let color = match pos_data[to_index(i,j, w) as usize].z {
                x if x < 0.0 => blue + darker,
                x if x >= 5.0 => white + light,
                _ =>  green + dark,
            };

            tex_coords.insert(format!("{},{}", color.x, color.y));
            tex_data.push([color.x, color.y]);
        }
    }

    println!("{:?}", tex_coords);
    //panic!();
    tex_data
}


fn to_index(i: u32, j: u32, w: u32) -> u32 {
    i * w + j
}
