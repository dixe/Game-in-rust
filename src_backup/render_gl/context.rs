use nalgebra as na;
use crate::render_gl;

use crate::resources::Resources;
use std::path::Path;

pub struct Context {
    pub res: Resources,
    pub sdl: sdl2::Sdl,


    pub video_subsystem: sdl2::VideoSubsystem,
    pub controller_subsystem: sdl2::GameControllerSubsystem,
    pub window:  sdl2::video::Window,

    pub gl_context:  sdl2::video::GLContext,
    pub gl: gl::Gl,


    pub transform: na::Matrix4<f32>,
    pub viewport: render_gl::Viewport,

    controller: Option<sdl2::controller::GameController>,
    wire_frame: bool,

}


impl Context {
    pub fn switch_mode(&mut self){
        if self.wire_frame {
            unsafe {
                self.gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
        }
        else{
            unsafe {
                self.gl.PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }

        self.wire_frame = !self.wire_frame
    }


    pub fn gl_swap_window(&self) {

        self.window.gl_swap_window();
    }


    pub fn set_controller(&mut self, which: u32) {
        let ctrl = self.controller_subsystem.open(which).unwrap();

        println!("Added game controller: {}\nAttached: {}\nPolling; {}", which, ctrl.attached(),
                 self.controller_subsystem.event_state());
        self.controller = Some(ctrl);
    }

}



pub fn setup() -> Result<Context, failure::Error>
{
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let controller_subsystem = sdl.game_controller().unwrap();

    controller_subsystem.set_event_state(true);

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);


    let viewport = render_gl::Viewport::for_window(900, 700);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()?;

    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s|{
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    viewport.set_used(&gl);

    let wire_frame = true;

    let transform = na::Matrix4::identity();

    unsafe {
        gl.Enable(gl::DEPTH_TEST);
    }

    Ok(Context{
        res,
        sdl,
        video_subsystem,
        controller_subsystem ,
        gl_context,
        window,
        gl,
        wire_frame,
        transform,
        viewport,
        controller: None,
    })
}
