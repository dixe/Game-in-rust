use sdl2::keyboard::Keycode;
use na;
use gl;

use crate::game;
use crate::cube;
use crate::controls;
use crate::render_gl;
use crate::physics;

pub struct PhysicsTest {
    pub static_cube: cube::Cube,
    pub cube: cube::Cube,
    pub static_pos: na::Vector3::<f32>,
    pub pos: na::Vector3::<f32>,
    pub rot: na::Vector3::<f32>,

    pub col: bool
}


impl PhysicsTest {

    pub fn new(gl: &gl::Gl) -> PhysicsTest {

        let clr = na::Vector3::new(0.0, 1.0, 0.0);

        let static_clr = na::Vector3::new(0.0, 0.0, 1.0);

        let static_cube = cube::Cube::new(static_clr, gl);

        let cube = cube::Cube::new(clr, gl);


        PhysicsTest {
            static_cube: cube::Cube::new(static_clr, gl),
            cube: cube::Cube::new(clr, gl),
            static_pos: na::Vector3::new(-5.0, 0.0, 2.0),
            pos: na::Vector3::new(-5.0, 2.0, 2.0),
            rot: na::Vector3::new(0.0, 0.0, 0.0),
            col: false
        }
    }


    pub fn update(&mut self, ctl: &controls::Controls, delta: f32) {
        if ctl.left {
            self.pos.y += 1.0 * delta;
        }

        if ctl.right {
            self.pos.y -= 1.0 * delta;
        }

        if ctl.up {
            self.pos.x += 1.0 * delta;
        }

        if ctl.down {
            self.pos.x -= 1.0 * delta;
        }


        if ctl.q {
            self.pos.z -= 1.0 * delta;
        }
        if ctl.e {
            self.pos.z += 1.0 * delta;
        }


        ctl.keys.get(&Keycode::X).map(|is_set| {

            if *is_set {
                self.rot.x += 1.0 * delta;
            }
            is_set
        });

        ctl.keys.get(&Keycode::Y).map(|is_set| {

            if *is_set {
                self.rot.y += 1.0 * delta;
            }
            is_set
        });

        ctl.keys.get(&Keycode::Z).map(|is_set| {

            if *is_set {
                self.rot.z += 1.0 * delta;
            }
            is_set
        });




        let static_rot = na::Rotation3::new(na::Vector3::new(0.0, 0.0, 0.0));
        let static_scale = na::Matrix3::identity();
        let static_cb = physics::CollisionBox::new(self.static_pos, static_rot, static_scale);

        let rot = na::Rotation3::new(self.rot);
        let scale = na::Matrix3::identity();
        let cb = physics::CollisionBox::new(self.pos, rot, scale);

        self.col = physics::check_collision(&static_cb, &cb);
    }


    pub fn render(&self, ctx: &game::Context, shader: &render_gl::Shader) {

        shader.set_used();

        // CAN BE MOVED OUTSIDE THE LOOP

        shader.set_projection_and_view(&ctx.render_context.gl, ctx.camera().projection(), ctx.camera().view());


        let model_static = na::Matrix4::new_translation(&self.static_pos);

        let rot_mat = na::Matrix4::<f32>::new_rotation(self.rot);
        let model = na::Matrix4::new_translation(&self.pos);

        // static cube
        let static_color = na::Vector3::new(1.0, 1.0, 1.0);
        shader.set_vec3(&ctx.render_context.gl, "color", static_color);
        self.static_cube.render(&ctx.render_context.gl, &ctx.cube_shader, model_static);


        // dynamic cube
        let mut color = na::Vector3::new(1.0, 1.0, 1.0);

        if self.col {
            color = na::Vector3::new(1.0, 0.0, 0.0);
        }
        shader.set_vec3(&ctx.render_context.gl, "color", color);
        self.cube.render(&ctx.render_context.gl, shader, model * rot_mat);

    }

}
