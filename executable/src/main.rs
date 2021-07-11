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
extern crate image;
extern crate gltf;
extern crate shared;


use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use std::io;
use std::thread;

pub mod render_gl;
pub mod resources;

pub mod text_render;
pub mod types;

pub mod triangle;
pub mod square;
pub mod cube;
pub mod floor;
pub mod level;
pub mod controls;
mod debug;

mod math;
mod game;
mod deltatime;
mod entity;
mod camera;
mod physics;
mod action_system;

mod test_physics;


#[derive(Copy, Clone, Debug)]
enum Command {
    Nop,
    SwitchRenderMode,
    ReloadAssets,
    ReloadAi,
    Quit,
    DecrementWalkTime,
    IncrementWalkTime,
}


static mut CMD: Command = Command::Nop;



fn start_cmd_thread() {
    thread::spawn(move || {

        loop {

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
                        Command::ReloadAssets
                    }
                    ,
                    "m" => Command::SwitchRenderMode,
                    "q" => Command::Quit,
                    "d" => Command::DecrementWalkTime,
                    "w" => Command::IncrementWalkTime,
                    _ => Command::Nop,
                };

                CMD = msg;

            }
        }
    });
}

fn start_notify_thread_assets() {
    thread::spawn(move || {
        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Create a watcher object, delivering debounced events.
        // The notification back-end is selected based on the platform.
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch("E:/repos/Game-in-rust/assets/", RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
                Ok(_) => {
                    println!("Updated on disk copy assets");
                    copy_assets();
                    unsafe {
                        CMD = Command::ReloadAssets;
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}


fn start_notify_thread_ais() {
    thread::spawn(move || {
        // Create a channel to receive the events.
        let (tx, rx) = channel();

        // Create a watcher object, delivering debounced events.
        // The notification back-end is selected based on the platform.
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch("E:/repos/Game-in-rust/target/debug/ai.dll", RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
                Ok(_) => {
                    println!("Updated AI disk copy assets");
                    copy_assets();
                    unsafe {
                        CMD = Command::ReloadAi;
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
    start_notify_thread_assets();

    // monitor ais on disk and reload them
    start_notify_thread_ais();

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

    let mut ctx = game::Context::new()?;

    let mut physics_test = test_physics::PhysicsTest::new(&ctx.render_context.gl);

    let collision_shader = render_gl::Shader::new("collision_test_shader", &ctx.render_context.res, &ctx.render_context.gl)?;

    // setup texture
    let low_poly_texture_id = render_gl::texture::load_and_set("low_poly.png", &ctx.render_context.res, &ctx.render_context.gl)?;

    let bone_cube = cube::Cube::new(na::Vector3::new(0.5, 0.5, 0.5), &ctx.render_context.gl);

    let ft = text_render::free_type_wrapper::load_free_type();

    let bitmaps = text_render::bitmap_generator::generate_map(&ft, &ctx.render_context.gl);

    let bitmap_quad = text_render::bitmap_generator::generate_quad(&ctx.render_context.gl);



    'main: loop{
        ctx.update_delta();
        // For debugging always set delta to 0.016 (60fps) even when having long breaks in visualStudio
        //let delta = ctx.get_delta_time();
        let delta = 0.016;
        ctx.handle_inputs();


        if ctx.controls.quit {
            break 'main;
        }

        if ctx.controls.reset {
            ctx.scene.entities.player.base_entity.physics = shared::Physics::new();

            ctx.scene.entities.player.base_entity.physics.pos.x = 0.0;
            ctx.scene.entities.player.base_entity.physics.pos.y = 0.0;
            ctx.scene.entities.player.base_entity.physics.pos.z = 10.0;

            ctx.scene.entities.player.skeleton.reset_ik();

            for enemy in ctx.scene.entities.enemies.values_mut() {
                enemy.base_entity.physics = shared::Physics::new();
                enemy.skeleton.reset_ik();
            }
        }

        unsafe {
            ctx.render_context.gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }


        game::ai::run_ais(&mut ctx.scene);

        //PHYSICS PROCESSING
        let collisions = physics::process(&mut ctx.scene, delta);

        // SPAWN PROJECTILES, HANDLE COLLISION THAT WAS NOT WITH ENVIROMENT
        game::update_game_state(&mut ctx.scene, &ctx.controls, &collisions);



        //println!("FPS: {:?}", 1.0 / delta);
        //println!("{:?}", ctx.scene.entities.player.base_entity.physics.velocity.magnitude());

        //UPDATE CAMERA IF FOLLOW MODE

        let mode = ctx.scene.camera().mode();
        match mode {
            camera::CameraMode::Free => {
                update_free_camera(&mut ctx, delta);
            },
            camera::CameraMode::Follow => {
                update_follow_camera(&mut ctx.scene, &ctx.controls);
            },
        };


        //PHYSICS TEST
        physics_test.update(&ctx.controls, delta);
        physics_test.render(&ctx.render_context.gl, &ctx.scene, &collision_shader);



        // DEBUG
        debug_keys(&mut ctx, &bone_cube);

        // ANIMATIONS UPDATE
        ctx.scene.update_animations(delta);
        // RENDERING
        ctx.scene.render(&mut ctx.render_context, low_poly_texture_id, &bitmaps, &bitmap_quad);

        ctx.render_context.gl_swap_window();


        unsafe {
            match CMD {
                Command::Nop => {continue},
                Command::Quit => { break 'main},
                Command::ReloadAssets => {
                    println!("Reload assets");
                    ctx.scene.reload_shaders(&ctx.render_context);
                },
                Command::ReloadAi => {
                    println!("Reload Ais");
                    ctx.scene.reload_ais(&ctx.res_dll);
                },
                Command::SwitchRenderMode => {
                    ctx.render_context.switch_mode();
                },

                Command::DecrementWalkTime => {
                    let player = &mut ctx.scene.entities.player;
                    player.animation_player.as_mut().map(|ap| ap.animations.walk.duration -= 0.1);
                    println!("{:#?}", player.animation_player.as_ref().map(|ap| ap.animations.walk.duration));
                },
                Command::IncrementWalkTime => {
                    let player = &mut ctx.scene.entities.player;
                    player.animation_player.as_mut().map(|ap| ap.animations.walk.duration += 0.1);
                    println!("{:#?}", player.animation_player.as_ref().map(|ap| ap.animations.walk.duration));
                },
            }
            CMD = Command::Nop;
        }

    }

    Ok(())
}


fn debug_keys(ctx: &mut game::Context, bone_cube: &cube::Cube) {

    let player = &mut ctx.scene.entities.player;

    let animation_player = player.animation_player.as_mut().unwrap();

    let bones = player.bones.clone();

    let skeleton = player.skeleton.clone();

    match ctx.controls.keys.get(&sdl2::keyboard::Keycode::B) {
        Some(true) => {
            println!("BONES");
            println!("there are {:#?} bones", bones.len());
            println!("there are {:#?} skel joints", skeleton.joints.len());
            for i in 0..bones.len() {
                println!("i = {} {:#?}", skeleton.joints[i].name.clone(), bones[i]);
            }
        },
        _ => {}
    }

    match ctx.controls.keys.get(&sdl2::keyboard::Keycode::T) {
        Some(true) => {
            animation_player.set_current(render_gl::Animation::TPose, &skeleton);

        },
        _ => {}
    };

    match ctx.controls.keys.get(&sdl2::keyboard::Keycode::K) {
        Some(true) => {
            println!("Setting to waalk");
            animation_player.set_current(render_gl::Animation::Walk, &skeleton);
        },
        _ => {
        }

    };

    //HIT BOXES
    match ctx.controls.keys.get(&sdl2::keyboard::Keycode::H) {
        Some(true) => {
            ctx.scene.render_hitboxes = true;
        },
        _ => {
        }

    };

    //HIT BOXES
    match ctx.controls.keys.get(&sdl2::keyboard::Keycode::P) {
        Some(true) => {
            for hitbox_base in &ctx.scene.entities.player.hitboxes {
                let hitbox = hitbox_base.make_transformed(ctx.scene.entities.player.base_entity.physics.pos, ctx.scene.entities.player.base_entity.physics.rotation);

                println!("hitbox max_x, min_x, max_y, min_y, max_z, min_z {} {} {} {} {} {}",
                         hitbox.max_x(), hitbox.min_x(),
                         hitbox.max_y(), hitbox.min_y(),
                         hitbox.max_z(), hitbox.min_z() );
                println!("player_pos x y z {} {} {}", ctx.scene.entities.player.base_entity.physics.pos.x, ctx.scene.entities.player.base_entity.physics.pos.y, ctx.scene.entities.player.base_entity.physics.pos.z);
                println!("player_rot {:?}", ctx.scene.entities.player.base_entity.physics.rotation.to_euler_angles());
            }
        },
        _ => {
        }

    };

    match ctx.controls.keys.get(&sdl2::keyboard::Keycode::V) {
        Some(true) => {

            ctx.scene.cube_shader.set_used();
            let proj = ctx.scene.camera().projection();
            let view = ctx.scene.camera().view();
            ctx.scene.cube_shader.set_projection_and_view(&ctx.render_context.gl, proj, view);


            let mut scale_mat = na::Matrix4::identity();
            scale_mat = scale_mat * 0.2;
            scale_mat[15] = 1.0;


            for joint in &skeleton.joints {
                bone_cube.render(&ctx.render_context.gl, &ctx.scene.cube_shader, joint.world_matrix * scale_mat);
            }

        },
        _ => {}
    };

}

fn set_t_pose(bones: &mut [na::Matrix4::<f32>]) {
    for i in 0..bones.len() {
        bones[i] = na::Matrix4::identity();
    }
}

fn update_follow_camera(scene: &mut game::Scene, controls: &controls::Controls) {

    let player = &scene.entities.player;

    let mut physics = player.base_entity.physics;

    // pos.z bottom of player model
    physics.pos.z += 1.6;


    // readjust to player
    scene.camera_mut().update_target(physics.pos);

    // camera movement by stick
    controls.right_stick.map(|right_stick| {
        scene.camera_mut().update_movement(right_stick.x, right_stick.y);
    });

}

fn update_free_camera(ctx: &mut game::Context, delta: f32) {

    use sdl2::keyboard::Keycode;

    let mut move_dir = ctx.controls.movement_dir;

    if ctx.scene.camera().mode() == camera::CameraMode::Free {
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

    ctx.scene.camera_mut().move_camera(move_dir, delta);

    let mouse_move = ctx.controls.mouse_move;

    /*
    let mut new_pos = ctx.scene.camera_mut().pos();

    new_pos.x = ctx.scene.entities.player.base_entity.physics.pos.x;

    ctx.scene.camera_mut().set_pos(new_pos);

     */

    ctx.scene.camera_mut().update_movement(mouse_move.x, mouse_move.y);
}
