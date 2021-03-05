use nalgebra as na;

use crate::entity;
use crate::render_gl;
use crate::cube;
use crate::camera;
use crate::scene;
use crate::level;
use crate::controls;


pub struct Context {

    pub player_projectiles: Vec::<entity::Entity>,

    pub player: entity::Entity,
    pub enemies: Vec::<entity::Entity>,
    pub controls: controls::Controls,
    pub scene: scene::Scene,
    pub level: level::Level,
    pub render_context: render_gl::context::Context,
    pub camera: camera::Camera,


}

impl Context {

    pub fn new() -> Result<Context, failure::Error> {


        let render_context = render_gl::context::setup()?;

        let background_color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));

        let player_pos = na::Vector3::new(3.0, 3.0, 0.0);
        let player_color = na::Vector3::new(0.0,  1.0, 1.0);

        let enemy_pos = na::Vector3::new(0.0, 0.0, 0.0);

        let enemy_color = na::Vector3::new(0.0, 0.0, 0.0);

        background_color_buffer.set_used(&render_context.gl);

        let camera = camera::Camera::new();

        let event_pump = render_context.sdl.event_pump().unwrap();

        let level = level::Level::load(&render_context.res,"levels/debugLevel1.txt")?;

        let scene = scene::Scene::new(&level, &render_context)?;

        let controls = controls::Controls::new(event_pump);

        let enemy_cube = cube::Cube::new(&render_context.res, enemy_color, &render_context.gl)?;
        let enemy1 = entity::Entity::new(enemy_cube, enemy_pos);

        let player_cube = cube::Cube::new(&render_context.res, player_color, &render_context.gl)?;
        let player = entity::Entity::new(player_cube, player_pos);

        Ok(Context {
            player_projectiles: Vec::<entity::Entity>::new(),
            enemies: vec! [enemy1],
            player,
            scene,
            controls,
            render_context,
            level,
            camera
        })
    }


    pub fn handle_inputs(&mut self) {
        self.controls.handle_inputs(&mut self.render_context);
    }

    pub fn render(&self){


        self.scene.render(&self.render_context.gl, self.camera.projection(), self.camera.view());
        self.player.render(&self.render_context.gl, self.camera.projection(), self.camera.view());

        for e in &self.enemies {
            e.render(&self.render_context.gl, self.camera.projection(), self.camera.view());
        }

        self.render_context.window.gl_swap_window();
    }

}
