use nalgebra as na;


use crate::physics;
use crate::cube;
use crate::entity;
use crate::render_gl;

use crate::camera;
use crate::level;
use crate::controls;
use crate::deltatime;
use crate::action_system;
use crate::game;


pub struct Context {

    pub controls: controls::Controls,
    pub scene: game::Scene,
    pub render_context: render_gl::context::Context,

    delta_time: deltatime::Deltatime,

}

impl Context {

    pub fn new() -> Result<Context, failure::Error> {

        let mut ctx = empty()?;

        Ok(ctx)
    }



    // Call once pr update step
    pub fn update_delta(&mut self) {
        self.delta_time.update();
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time.time()
    }


    pub fn handle_inputs(&mut self) {
        let action = self.controls.handle_inputs(&mut self.render_context, &mut self.scene.cameras);

        match action {
            controls::Action::AddEnemy => {},
            controls::Action::NoAction => { },
        };
    }
}




fn empty() -> Result<Context, failure::Error> {

    let width = 900;
    let height = 700;

    let render_context = render_gl::context::setup(width, height)?;

    let background_color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));

    background_color_buffer.set_used(&render_context.gl);

    let event_pump = render_context.sdl.event_pump().unwrap();
    let controls = controls::Controls::new(event_pump);

    let scene = game::Scene::new(&render_context).unwrap();

    let delta_time = deltatime::Deltatime::new();



    Ok(Context {
        scene,
        controls,
        render_context,
        delta_time,
    })
}
