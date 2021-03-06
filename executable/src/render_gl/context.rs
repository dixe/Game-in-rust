
use crate::render_gl;

use crate::resources::Resources;
use std::path::Path;

pub struct Context {
    pub res: Resources,
    pub sdl: sdl2::Sdl,


    pub video_subsystem: sdl2::VideoSubsystem,
    pub controller_subsystem: sdl2::GameControllerSubsystem,
    pub window: sdl2::video::Window,

    pub gl_context: sdl2::video::GLContext,
    pub gl: gl::Gl,


    pub viewport: render_gl::Viewport,

    controller: Option<sdl2::controller::GameController>,
    pub wire_frame: bool,

}


impl Context {
    pub fn switch_mode(&mut self){

        self.wire_frame = !self.wire_frame;

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



pub fn setup(width: u32, height: u32) -> Result<Context, failure::Error>
{
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    //sdl.mouse().show_cursor(false);

    //sdl.mouse().set_relative_mouse_mode(true);

    let controller_subsystem = sdl.game_controller().unwrap();

    controller_subsystem.set_event_state(true);

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);


    let viewport = render_gl::Viewport::for_window(width as i32, height as i32);

    let window = video_subsystem
        .window("Game", width, height)
        .opengl()
        .resizable()
        .build()?;



    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s|{
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    viewport.set_used(&gl);

    let wire_frame = false;

    unsafe {
        gl.Enable(gl::DEPTH_TEST);
        gl.Enable(gl::BLEND);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
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
        viewport,
        controller: None,
    })
}
