use sdl2::keyboard::Keycode;
use na;
use gl;

use crate::game;
use crate::cube;
use crate::controls;

pub struct PhysicsTest {
    pub static_cube: cube::Cube,
    pub cube: cube::Cube,
    pub static_pos: na::Vector3::<f32>,
    pub pos: na::Vector3::<f32>,
    pub rot: na::Vector3::<f32>,
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

    }


    pub fn render(&self, ctx: &game::Context) {

        let model_static = na::Matrix4::new_translation(&self.static_pos);

        let rot_mat = na::Matrix4::<f32>::new_rotation(self.rot);
        let model = na::Matrix4::new_translation(&self.pos);
        self.static_cube.render(&ctx.render_context.gl, &ctx.cube_shader, model_static);
        self.cube.render(&ctx.render_context.gl, &ctx.cube_shader, model * rot_mat);

    }

}
