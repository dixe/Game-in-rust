extern crate sdl2;
extern crate gl;
extern crate vec_2_10_10_10;
extern crate nalgebra as na;
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;
#[macro_use] extern crate entity_component_derive;


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


        /*
        let enemy_color = na::Vector3::new(1.0, 1.0, 1.0);
        let cube = cube::Cube::new(&ctx.render_context.res, enemy_color, &ctx.render_context.gl)?;

        let mut model = entity::Model::new(cube);

        model.scale(&na::Vector3::new(5.0, 5.0, 5.0));
        let pos = na::Vector3::new(0.0, 0.0, 0.0);
        model.render(&ctx.render_context.gl, &ctx.camera.projection(), &ctx.camera.view(), pos);


         */
        ctx.render_context.gl_swap_window();

    }

    Ok(())
}
