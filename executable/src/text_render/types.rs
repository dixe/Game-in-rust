use crate::render_gl::{buffer};
use crate::types::*;


pub struct Character {
    pub texture_id: u32,
    pub size: V2,
    pub bearing: V2,
    pub advance: u32
}


pub struct BitmapQuad {
    pub vao: buffer::VertexArray,
    pub vbo: buffer::ArrayBuffer,
}
