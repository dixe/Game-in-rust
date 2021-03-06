use std::os::raw;
use failure;
use gl;

use crate::resources::Resources;

pub fn load_and_set(name: &str, res: &Resources, gl: &gl::Gl) -> Result<u32, failure::Error> {

    let prefix = "palettes/".to_owned();
    let path = prefix + name;

    let img = res.load_image_rgb8(&path)?;

    let mut obj: gl::types::GLuint = 0;
    unsafe {
        gl.GenTextures(1, &mut obj);

        gl.BindTexture(gl::TEXTURE_2D, obj);

        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);

        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 8, 8, 0, gl::RGB, gl::UNSIGNED_BYTE, img.as_ptr() as *const raw::c_void);
    }

    Ok(obj)
}


pub fn set_texture(gl: &gl::Gl, texture_id: u32) {
    unsafe {
        gl.BindTexture(gl::TEXTURE_2D, texture_id);
    }
}

pub fn bitmap_texture(gl: &gl::Gl, bytes: &[u8], width: i32, height: i32) -> Result<u32, failure::Error> {


    let mut obj: gl::types::GLuint = 0;
    unsafe {
        gl.PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        gl.GenTextures(1, &mut obj);
        gl.BindTexture(gl::TEXTURE_2D, obj);

        gl.TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as i32,
            width,
            height,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            bytes.as_ptr() as *const raw::c_void);


        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);


    }

    Ok(obj as u32)
}
