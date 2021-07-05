use gl;
use crate::entity;
use crate::render_gl;
use crate::types::*;
use crate::text_render;
use crate::types::*;

pub fn render_entity(entity: &entity::Entity, model: &entity::Model, gl: &gl::Gl, shader: &render_gl::Shader) {
    model.render_from_model_mat(gl, shader, entity.base_entity.physics.calculate_model_mat(), &entity.bones);
}




pub fn render_world(model: &entity::Model, gl: &gl::Gl, shader: &render_gl::Shader) {
    model.render_from_model_mat(gl, shader, na::Matrix4::identity(), &Vec::new());
}



pub fn render_text(gl: &gl::Gl,
                   charMap: &std::collections::HashMap<u32, text_render::Character>,
                   shader: &mut render_gl::Shader,
                   vao: &render_gl::buffer::VertexArray,
                   vbo: &render_gl::buffer::ArrayBuffer,
                   color: &V3) {

    let text = "test Text";

    unsafe {
        shader.set_used();
        shader.set_vec3(gl, "textColor", *color);
        gl.ActiveTexture(gl::TEXTURE0);
        vao.bind();
    }


    let mut x = 10.0;

    let mut y = 10.0;

    for text_c in text.chars() {

        let ch = charMap.get(&(text_c as u32)).unwrap();

        // TODO scale
        let xpos = x + ch.bearing.x;
        let ypos = y - (ch.size.y - ch.bearing.y);
        let w = ch.size.x;
        let h = ch.size.y;

        let vertices = vec![
            xpos,     ypos + h,   0.0, 0.0,
            xpos,     ypos,       0.0, 1.0,
            xpos + w, ypos,       1.0, 1.0,

            xpos,     ypos + h,   0.0, 0.0,
            xpos + w, ypos,       1.0, 1.0,
            xpos + w, ypos + h,   1.0, 0.0
        ];


        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, ch.texture_id);

            vbo.bind();
            gl.BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid
            );


            gl.BindBuffer(gl::ARRAY_BUFFER, 0);

            gl.DrawArrays(gl::TRIANGLES, 0, 6);

            //TODO scale
            x += (ch.advance >> 6) as f32;

        }

    }

    unsafe {
        vao.unbind();
        gl.BindTexture(gl::TEXTURE_2D, 0);
    }
}
