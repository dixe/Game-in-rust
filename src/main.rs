extern crate sdl2;
extern crate gl;
extern crate vec_2_10_10_10;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;
#[macro_use] extern crate entity_component_derive;

use std::io;
use std::thread;

pub mod render_gl;
pub mod resources;

pub mod triangle;
pub mod square;
pub mod cube;
pub mod floor;
pub mod level;
pub mod controls;
mod debug;

mod game;
mod deltatime;
mod shot;
mod entity;
mod camera;
mod scene;
mod physics;


#[derive(Copy, Clone, Debug)]
enum Command {
    Nop,
    SwitchRenderMode,
}


static mut CMD: Command = Command::Nop;

fn main() {
    // set up commands channel and thread
    thread::spawn(move || {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");


        unsafe {
            // parse input
            let msg = match input {
                _ => Command::SwitchRenderMode,
            };

            CMD = msg;

        }
    });


    // start game
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}




fn run() -> Result<(), failure::Error> {

    let mut ctx = game::Context::new()?;
    /*
    let light_color = na::Vector3::new(1.0, 1.0, 1.0);
    let light_cube = cube::Cube::new(light_color, &ctx.render_context.gl);

    let light_model = entity::Model::new(light_cube);

    let triangle_pos = na::Vector3::new(0.0, 0.0, 5.0);

    let tri = triangle::Triangle::new(&ctx.render_context.res, pos, light_color, &ctx.render_context.gl)?;

    let pos = na::Vector3::new(0.0, 0.0, 15.0);

     */
    'main: loop{

        ctx.update_delta();

        ctx.handle_inputs();

        if ctx.controls.quit {
            break 'main;
        }

        unsafe {
            ctx.render_context.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }


        game::run_ai(&mut ctx);

        //PHYSICS PROCESSING
        let collisions = physics::process(&mut ctx);

        // SPAWN PROJECTILES, HANDLE COLLISION THAT WAS NOT WITH ENVIROMENT
        game::update_game_state(&mut ctx, &collisions);


        // RENDERING
        ctx.render();




        ctx.light_shader.set_projection_and_view(&ctx.render_context.gl, ctx.camera.projection(), ctx.camera.view());
        //light_model.render(&ctx.render_context.gl, &ctx.light_shader, pos);

        //tri.render(&ctx.render_context.gl);

        ctx.render_context.gl_swap_window();

        unsafe {
            match CMD {
                Command::Nop => {},
                Command::SwitchRenderMode => {
                    CMD = Command::Nop;
                    ctx.render_context.switch_mode();


                },

            }
        }
    }
    Ok(())
}
