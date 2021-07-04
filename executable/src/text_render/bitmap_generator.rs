use freetype::face::LoadFlag;
use crate::text_render::*;
use gl;
use crate::text_render;
use crate::render_gl;
use crate::types::*;


//TODO should be in render gl since we wil be using gl and textures
pub fn generate_map(ft: &text_render::free_type_wrapper::FreeTypeWrapper, gl: &gl::Gl) -> std::collections::HashMap<u8, Character> {

    let mut res = std::collections::HashMap::new();

    for c in 0..128 {

        ft.face.load_char(c, LoadFlag::RENDER).unwrap();

        let glyph = ft.face.glyph();

        let bitmap = glyph.bitmap();

        //TODO handle failure results
        let texture_id = render_gl::texture::bitmap_texture(gl, bitmap.buffer(), bitmap.width(), bitmap.rows()).unwrap();


        let character = Character {
            texture_id,
            size: V2i::new(bitmap.width(), bitmap.rows()),
            bearing: V2i::new(glyph.bitmap_left(), glyph.bitmap_top()),
            advance: glyph.advance().x as u32
        };

        res.insert(c as u8, character);
    }

    res

}
