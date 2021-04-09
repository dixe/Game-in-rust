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
extern crate collada;

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;



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
                Ok(_event) => {
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

struct MeshAndSkeleton {

    mesh: render_gl::SkinnedMesh,
    skeleton: render_gl::Skeleton,
}


fn load_collada(gl: &gl::Gl) -> Result<MeshAndSkeleton, failure::Error> {

    let path = std::path::Path::new("E:/repos/Game-in-rust/blender_models/player_02.dae");

    let doc: collada::document::ColladaDocument = collada::document::ColladaDocument::from_path(path).unwrap();


    let mesh = render_gl::SkinnedMesh::from_collada(&doc, gl, "cube");

    let mut skeleton = render_gl::Skeleton::from_collada(&doc, &mesh.skeleton_name);

    Ok(MeshAndSkeleton {
        mesh,
        skeleton
    })
}


fn set_t_pose(skeleton: &mut render_gl::Skeleton, index: usize) {

    let joint = &skeleton.joints[index];

    let local_matrix = joint.get_base_local_matrix();

    let mut world_matrix = local_matrix;

    if joint.parent_index != 255 {
        world_matrix = skeleton.joints[joint.parent_index].world_matrix * local_matrix;
    }

    if joint.parent_index >= index && joint.parent_index != 255 {
        panic!("Bones are not in correct order. All children should be after parent current {}, parent {}", index, joint.parent_index);
    }

    println!("Index: {} - name: {}", index, joint.name.clone());

    let name = joint.name.clone();


    skeleton.joints[index].world_matrix = world_matrix;
    skeleton.joints[index].inverse_bind_pose = world_matrix.try_inverse().unwrap();

}

fn set_bone(bones: &mut [na::Matrix4::<f32>], skeleton: &mut render_gl::Skeleton, index: usize,  rot: na::UnitQuaternion::<f32>, trans: na::Vector3::<f32>) {

    let joint = &skeleton.joints[index];

    let local_matrix = joint.get_local_matrix(rot, trans);

    let mut world_matrix = local_matrix;

    if joint.parent_index != 255 {
        world_matrix = skeleton.joints[joint.parent_index].world_matrix * local_matrix;
    }

    bones[index] = world_matrix * joint.inverse_bind_pose;
    skeleton.joints[index].world_matrix = world_matrix;



}


fn run() -> Result<(), failure::Error> {


    let mut ctx = game::Context::new()?;

    let mut physics_test = physics_test::PhysicsTest::new(&ctx.render_context.gl);

    let collision_shader = render_gl::Shader::new("collision_test_shader", &ctx.render_context.res, &ctx.render_context.gl)?;

    let mut mesh_shader = render_gl::Shader::new("mesh_shader", &ctx.render_context.res, &ctx.render_context.gl)?;

    let mesh_and_skeleton = load_collada(&ctx.render_context.gl)?;

    let mesh = mesh_and_skeleton.mesh;
    let mut skeleton = mesh_and_skeleton.skeleton;

    let bone_cube = cube::Cube::new(na::Vector3::new(0.5, 0.5, 0.5), &ctx.render_context.gl);


    let mut bones = Vec::new();

    let joint_count = skeleton.joints.len();

    for i in 0..=joint_count {
        bones.push(na::Matrix4::identity());
    }

    for i in 0..joint_count {
        set_t_pose(&mut skeleton, i);
    }

    println!("joints {:#?}", skeleton.joints.len());

    let trans = na::Vector3::new(0.0, 0.0, 0.0);


    for i in 0..joint_count {
        let mut rot = na::UnitQuaternion::identity();

        if i == 3 {
            rot = na::UnitQuaternion::from_euler_angles(-1.0, 0.0, 0.0);
        }

        if i == 4 {
            rot = na::UnitQuaternion::from_euler_angles(0.0, 0.0, 1.29);
        }

        set_bone(&mut bones, &mut skeleton, i, rot, trans);
    }




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

        let mode = ctx.camera().mode();
        match mode {
            camera::CameraMode::Free => {
                update_free_camera(&mut ctx);
            },
            camera::CameraMode::Follow => {
                update_follow_camera(&mut ctx);
            },
        };



        //PHYSICS TEST
        //physics_test.update(&ctx.controls, ctx.get_delta_time());

        //physics_test.render(&ctx, &collision_shader);



        // RENDERING
        //ctx.render();

        ctx.cube_shader.set_used();

        ctx.cube_shader.set_projection_and_view(&ctx.render_context.gl, ctx.camera().projection(), ctx.camera().view());

        let mut scale_mat = na::Matrix4::identity();
        scale_mat = scale_mat * 0.2;
        scale_mat[15] = 1.0;


        for joint in &skeleton.joints {
            bone_cube.render(&ctx.render_context.gl, &ctx.cube_shader, joint.world_matrix * scale_mat);
        }


        mesh_shader.set_used();

        mesh_shader.set_vec3(&ctx.render_context.gl, "lightPos", na::Vector3::new(0.0, 0.0, 5.0)); //

        mesh_shader.set_vec3(&ctx.render_context.gl, "lightColor", na::Vector3::new(1.0, 1.0, 1.0));

        mesh_shader.set_projection_and_view(&ctx.render_context.gl, ctx.camera().projection(), ctx.camera().view());
        mesh.render(&ctx.render_context.gl, &mesh_shader, na::Matrix4::identity(), &bones);

        ctx.render_context.gl_swap_window();

        unsafe {
            match CMD {
                Command::Nop => {continue},
                Command::Quit => { break 'main},
                Command::ReloadActions => {
                    println!("Reload action");

                    match render_gl::Shader::new("mesh_shader", &ctx.render_context.res, &ctx.render_context.gl) {
                        Ok(shader) => {
                            println!("Reloaded mesh shader");
                            mesh_shader = shader;
                        },
                        Err(err) => {
                            println!("Error loading mesh shader: {}",err);
                        }
                    };

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


fn update_follow_camera(ctx: &mut game::Context) {


    let mut player = match ctx.ecs.get_physics(ctx.player_id) {
        Some(e) => *e,
        None => return, // No player no follow
    };

    // pos.z bottom of player model
    player.pos.z += 1.6;

    ctx.camera_mut().update_target(player.pos);



    ctx.controls.right_stick.map(|right_stick| {
        ctx.camera_mut().update_movement(right_stick.x, right_stick.y);
    });

}

fn update_free_camera(ctx: &mut game::Context) {

    use sdl2::keyboard::Keycode;


    let mut move_dir = ctx.controls.movement_dir;


    if ctx.camera().mode() == camera::CameraMode::Free {
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

    let delta = ctx.get_delta_time();
    ctx.camera_mut().move_camera(move_dir, delta);

    let mouse_move = ctx.controls.mouse_move;


    ctx.camera_mut().update_movement(mouse_move.x, mouse_move.y);
}
