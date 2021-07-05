use freetype::face::LoadFlag;
use gl;
use crate::text_render::*;
use crate::render_gl::{texture, buffer};
use crate::types::*;


//TODO should be in render gl since we wil be using gl and textures
pub fn generate_map(ft: &free_type_wrapper::FreeTypeWrapper, gl: &gl::Gl) -> std::collections::HashMap<u32, Character> {

    let mut res = std::collections::HashMap::new();

    for c in 0..128 {

        ft.face.load_char(c, LoadFlag::RENDER).unwrap();

        let glyph = ft.face.glyph();

        let bitmap = glyph.bitmap();

        //TODO handle failure results
        let texture_id = texture::bitmap_texture(gl, bitmap.buffer(), bitmap.width(), bitmap.rows()).unwrap();


        let character = Character {
            texture_id,
            size: V2::new(bitmap.width() as f32, bitmap.rows() as f32),
            bearing: V2::new(glyph.bitmap_left() as f32, glyph.bitmap_top() as f32),
            advance: glyph.advance().x as u32
        };

        res.insert(c as u32, character);
    }
    res
}





pub fn generate_quad(gl: &gl::Gl) -> BitmapQuad {

    let vbo = buffer::ArrayBuffer::new(gl);
    let vao = buffer::VertexArray::new(gl);

    unsafe {

        vao.bind();

        vbo.bind();

        vbo.dynamic_draw_data((std::mem::size_of::<f32>() * 6 * 4) as u32);

        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
            0 as *const gl::types::GLvoid,
        );

        vbo.unbind();
        vao.unbind();

    }

    BitmapQuad {
        vbo: vbo,
        vao: vao,
    }


}
