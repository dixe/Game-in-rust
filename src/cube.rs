use gl;
use crate::render_gl::{self, data, buffer};
use nalgebra as na;
use crate::physics::CollisionBox;


#[derive(VertexAttribPointers,Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
    #[location = 2]
    normal: data::f32_f32_f32,
}


pub struct Cube {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
}

fn create_box(off_set: na::Vector3::<f32>, rotation: Option<na::Vector3::<f32>>) -> CollisionBox {
    let rot_mat = match rotation {
        Some(rot) => na::Rotation3::new(rot),
        None => na::Rotation3::identity(),
    };

    // println!("{:#?}", rot_mat);


    CollisionBox {
        v0: rot_mat * na::Vector3::new(0.0, 0.0, 0.0) + off_set,
        v1: rot_mat * na::Vector3::new(1.0, 0.0, 0.0) + off_set,
        v2: rot_mat * na::Vector3::new(1.0, 1.0, 0.0) + off_set,
        v3: rot_mat * na::Vector3::new(0.0, 1.0, 0.0) + off_set,
        v4: rot_mat * na::Vector3::new(0.0, 0.0, 1.0) + off_set,
        v5: rot_mat * na::Vector3::new(1.0, 0.0, 1.0) + off_set,
        v6: rot_mat * na::Vector3::new(1.0, 1.0, 1.0) + off_set,
        v7: rot_mat * na::Vector3::new(0.0, 1.0, 1.0) + off_set,
        name: "".to_string()
    }
}

impl Cube {



    pub fn new(clr: na::Vector3<f32>, gl: &gl::Gl) -> Cube {

        Cube::from_collision_box(create_box(na::Vector3::new(-0.5, -0.5, -0.5), None), clr, gl)
    }

    pub fn from_collision_box(collision_box: CollisionBox, clr: na::Vector3<f32>, gl: &gl::Gl) -> Cube {

        let vertices: Vec<f32> = vec![
            // vertecies             // colors          //normal
            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,    clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            collision_box.v1.x, collision_box.v1.y, collision_box.v1.z,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            collision_box.v2.x, collision_box.v2.y, collision_box.v2.z,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            collision_box.v2.x, collision_box.v2.y, collision_box.v2.z,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            collision_box.v3.x, collision_box.v3.y, collision_box.v3.z,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,     clr.x, clr.y, clr.z,     0.0,  0.0, -1.0,
            collision_box.v4.x, collision_box.v4.y, collision_box.v4.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            collision_box.v5.x, collision_box.v5.y, collision_box.v5.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            collision_box.v7.x, collision_box.v7.y, collision_box.v7.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            collision_box.v4.x, collision_box.v4.y, collision_box.v4.z,     clr.x, clr.y, clr.z,     0.0,  0.0,  1.0,
            collision_box.v7.x, collision_box.v7.y, collision_box.v7.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            collision_box.v3.x, collision_box.v3.y, collision_box.v3.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            collision_box.v4.x, collision_box.v4.y, collision_box.v4.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            collision_box.v7.x, collision_box.v7.y, collision_box.v7.z,     clr.x, clr.y, clr.z,    -1.0,  0.0,  0.0,
            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            collision_box.v2.x, collision_box.v2.y, collision_box.v2.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            collision_box.v1.x, collision_box.v1.y, collision_box.v1.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            collision_box.v1.x, collision_box.v1.y, collision_box.v1.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            collision_box.v5.x, collision_box.v5.y, collision_box.v5.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     1.0,  0.0,  0.0,
            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            collision_box.v1.x, collision_box.v1.y, collision_box.v1.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            collision_box.v5.x, collision_box.v5.y, collision_box.v5.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            collision_box.v5.x, collision_box.v5.y, collision_box.v5.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            collision_box.v4.x, collision_box.v4.y, collision_box.v4.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            collision_box.v0.x, collision_box.v0.y, collision_box.v0.z,     clr.x, clr.y, clr.z,     0.0, -1.0,  0.0,
            collision_box.v3.x, collision_box.v3.y, collision_box.v3.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            collision_box.v2.x, collision_box.v2.y, collision_box.v2.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            collision_box.v6.x, collision_box.v6.y, collision_box.v6.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            collision_box.v7.x, collision_box.v7.y, collision_box.v7.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0,
            collision_box.v3.x, collision_box.v3.y, collision_box.v3.z,     clr.x, clr.y, clr.z,     0.0,  1.0,  0.0
        ];


        let vbo = buffer::ArrayBuffer::new(gl);
        let vao = buffer::VertexArray::new(gl);

        let stride = 9;
        unsafe {
            // 1
            vao.bind();

            // 2.
            vbo.bind();
            vbo.static_draw_data(&vertices);

            // 4.
            // vertecies
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                0 as *const gl::types::GLvoid,
            );
            gl.EnableVertexAttribArray(0);

            // colors
            gl.VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );

            gl.EnableVertexAttribArray(1);

            // normals
            gl.VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint,
                (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );

            gl.EnableVertexAttribArray(2);

        }

        vbo.unbind();
        vao.unbind();


        Cube {
            vao,
            _vbo: vbo,
        }
    }

    pub fn render(&self, gl: &gl::Gl, shader: &render_gl::Shader, model: na::Matrix4<f32>,) {
        shader.set_model(gl, model);

        self.vao.bind();
        unsafe {

            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                36
            );
        }
    }
}
