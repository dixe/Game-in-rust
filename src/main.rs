extern crate sdl2;
extern crate gl;
extern crate vec_2_10_10_10;
extern crate nalgebra;
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;


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



fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}


fn run() -> Result<(), failure::Error> {

    let mut ctx = game::Context::new()?;



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
    }

    Ok(())
}
