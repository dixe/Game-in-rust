extern crate sdl2;
extern crate gl;
extern crate vec_2_10_10_10;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;
#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;
#[macro_use] extern crate entity_component_derive;
extern crate roxmltree;
extern crate notify;
extern crate walkdir;


use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use std::io;
use std::thread;
use fs_extra::dir;


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
mod entity;
mod camera;
mod scene;
mod physics;
mod action_system;

mod physics_test;

#[derive(Copy, Clone, Debug)]
enum Command {
    Nop,
    SwitchRenderMode,
    ReloadActions,
    Quit
}


static mut CMD: Command = Command::Nop;



fn start_cmd_thread() {
    thread::spawn(move || {

        while true {

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");


            unsafe {
                // parse input
                println!("Input is '{}'", input.trim());
                let msg = match input.trim() {
                    "r" =>{
                        copy_assets();
                        Command::ReloadActions
                    }
                    ,
                    "m" => Command::SwitchRenderMode,
                    "q" => Command::Quit,
                    _ => Command::Nop,
                };

                CMD = msg;

            }
        }
    });
}

fn start_notify_thread() {
    thread::spawn(move || {
        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Create a watcher object, delivering debounced events.
        // The notification back-end is selected based on the platform.
        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch("E:/repos/Game-in-rust/assets/", RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
                Ok(event) => {
                    println!("Updated on disk copy assets");
                    copy_assets();
                    unsafe {
                        CMD = Command::ReloadActions;
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}



fn main() {
    // set up commands channel and thread
    start_cmd_thread();


    // monitor assets on disk and reload them
    start_notify_thread();

    // start game
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}


fn copy_assets() {

    let mut options = fs_extra::dir::CopyOptions::new(); //Initialize default values for CopyOptions
    options.overwrite = true;


    // copy source/dir1 to target/dir1
    let copy_res = fs_extra::dir::copy("E:/repos/Game-in-rust/assets", "E:/repos/Game-in-rust/target/debug/", &options);

    match copy_res {
        Err(err) => println!("{:#?}", err),
        _ => {},
    };

}



fn run() -> Result<(), failure::Error> {

    let mut camera =  camera::FreeCamera::new();
    let mut ctx = game::Context::new(&mut camera)?;

    let mut physics_test = physics_test::PhysicsTest::new(&ctx.render_context.gl);

    let collision_shader = render_gl::Shader::new("collision_test_shader", &ctx.render_context.res, &ctx.render_context.gl)?;


    ctx.controls.cam_mode = controls::CameraMode::Free;

    'main: loop{
        ctx.update_delta();
        ctx.handle_inputs();


        if ctx.controls.quit {
            break 'main;
        }

        if ctx.controls.reset {
            let physics = entity::Physics::new(ctx.player_id);
            ctx.ecs.set_physics(ctx.player_id, physics);
        }

        unsafe {
            ctx.render_context.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }



        game::run_ai(&mut ctx);

        //PHYSICS PROCESSING
        let collisions = physics::process(&mut ctx);

        // SPAWN PROJECTILES, HANDLE COLLISION THAT WAS NOT WITH ENVIROMENT
        game::update_game_state(&mut ctx, &collisions);


        //UPDATE CAMERA IF FOLLOW MODE

        update_camera(&mut ctx);


        //PHYSICS TEST
        physics_test.update(&ctx.controls, ctx.get_delta_time());

        physics_test.render(&ctx, &collision_shader);



        // RENDERING
        ctx.render();

        ctx.render_context.gl_swap_window();

        unsafe {
            match CMD {
                Command::Nop => {continue},
                Command::Quit => { break 'main},
                Command::ReloadActions => {
                    //ctx.reload_actions();
                    ctx.load_model(ctx.player_weapon_id, na::Vector3::new(0.2, 0.2, 0.2), "models/sword.obj")?;
                },
                Command::SwitchRenderMode => {
                    ctx.render_context.switch_mode();
                },

            }
            CMD = Command::Nop;
        }

    }
    Ok(())
}



fn update_camera(ctx: &mut game::Context) {

    use sdl2::keyboard::Keycode;
    println!("{:#?}", ctx.controls.movement_dir);

    let mut move_dir = ctx.controls.movement_dir;

    if ctx.controls.cam_mode == controls::CameraMode::Free {
        ctx.controls.keys.get(&Keycode::LShift).map(|is_set| {
            if *is_set {
                move_dir.z += 1.0;
            }
        });

        ctx.controls.keys.get(&Keycode::LCtrl).map(|is_set| {
            if *is_set {
                move_dir.z -= 1.0;
            }
        });
    }

    ctx.camera.move_camera(move_dir, ctx.get_delta_time());
    ctx.camera.update_movement(ctx.controls.mouse_move.x, ctx.controls.mouse_move.y);
}

/*




if ctx.controls.cam_mode == controls::CameraMode::Follow {
let default = entity::Physics::new(0);
let physics = ctx.ecs.get_physics(ctx.player_id).unwrap_or(&default);


ctx.camera.set_target(physics.pos);


if ctx.controls.movement_dir.magnitude() > 0.0 && ctx.controls.right_stick.is_none(){

let z_rot =  physics.rotation.z;

/*println!("\n\n\n");
println!("BEHIND VEC: {:#?}", player_behind_vec);//ctx.camera.follow_dir);
println!("FOLLOW VEC: {:#?}", behind_xy);
println!("DIFF: {:#?}", diff);

 */

let mut rot_diff = ctx.camera.follow_yaw - (z_rot + 180.0_f32.to_radians());

if rot_diff < -std::f32::consts::PI {
rot_diff += 2.0 * std::f32::consts::PI;
}

    if rot_diff > std::f32::consts::PI {
    rot_diff -= 2.0 * std::f32::consts::PI;
}

    let smooth = 1.0;

    rot_diff = f32::min(smooth, f32::max(-smooth, rot_diff));
    let change_vec = na::Vector3::new(rot_diff, 0.0, 0.0) ;

    ctx.camera.change_follow_dir(change_vec);

}

    let right_stick = ctx.controls.right_stick;

    right_stick.map(|dir| ctx.camera.change_follow_dir(dir));
}

    if ctx.controls.cam_mode == controls::CameraMode::Free {

    let mut dir = na::Vector3::new(0.0, 0.0, 0.0);

    let speed = 5.0;
    if ctx.controls.w {
    dir.x += speed
}
    if ctx.controls.s {
    dir.x -= speed
}

    if ctx.controls.a {
    dir.y += speed
}
    if ctx.controls.d {
    dir.y -= speed
}

    ctx.camera.cam_target +=  dir * ctx.get_delta_time();

    ctx.camera.cam_pos += dir * ctx.get_delta_time();

    let sens = 0.005;
    ctx.camera.follow_yaw += ctx.controls.mouse_move.y * sens;
    ctx.camera.follow_pitch += ctx.controls.mouse_move.x * sens;

    ctx.camera.update_target();


}

}
     */
