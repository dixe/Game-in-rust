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

mod entity;
mod camera;
mod scene;
mod physics;


use nalgebra as na;


fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}


fn run() -> Result<(), failure::Error> {

    let mut render_context = render_gl::context::setup()?;

    let background_color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));

    let player_pos = na::Vector3::new(3.0, 3.0, 0.0);
    let player_color = na::Vector3::new(0.0,  1.0, 1.0);

    let enemy_pos = na::Vector3::new(0.0, 0.0, 0.0);
    let enemy_color = na::Vector3::new(0.0, 0.0, 0.0);


    let floor_clr = na::Vector3::new(0.50, 0.33, 0.6);

    let floor =  floor::Floor::new(&render_context.res, floor_clr, &render_context.gl)?;

    let player_cube = cube::Cube::new(&render_context.res, player_color, &render_context.gl)?;

    let enemy_cube = cube::Cube::new(&render_context.res, enemy_color, &render_context.gl)?;

    let mut player = entity::Entity::new(player_cube, player_pos);

    let mut enemy1 = entity::Entity::new(enemy_cube, enemy_pos);

    background_color_buffer.set_used(&render_context.gl);

    let camera = camera::Camera::new();

    let model_pos = na::Vector3::<f32>::new(0.0, 0.0, 0.0);

    let translation = na::Matrix4::new_translation(&model_pos);

    let model = translation *  na::Matrix4::identity();

    let event_pump = render_context.sdl.event_pump().unwrap();

    let level = level::Level::load(&render_context.res,"levels/debugLevel1.txt")?;

    let mut enemies = vec! [enemy1];

    let scene = scene::Scene::new(&level, &render_context)?;

    println!("{}", level);

    let mut controls = controls::Controls::new(event_pump);
    'main: loop{

        controls.handle_inputs(&mut render_context);

        if controls.quit {
            break 'main;
        }

        unsafe {
            render_context.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }


        //PHYSICS PROCESSING
        physics::process(&controls, &scene, &mut player, &mut enemies);

        //physics::process(&controls, &scene, &mut player);


        // RENDERING

        scene.render(&render_context.gl, camera.projection(), camera.view());
        floor.render(&render_context.gl, camera.projection(), camera.view(), model);
        player.render(&render_context.gl, camera.projection(), camera.view());

        for e in &enemies {
            e.render(&render_context.gl, camera.projection(), camera.view());
        }

        render_context.window.gl_swap_window();
    }

    Ok(())
}
