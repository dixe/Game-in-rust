use nalgebra as na;


use crate::render_gl::Renderable;
use crate::entity;
use crate::render_gl;
use crate::cube;
use crate::camera;
use crate::scene;
use crate::level;
use crate::controls;
use crate::deltatime;
use crate::shot;


pub struct Context {


    //GAME STATE SHOULD MOVE INTO STRUCT/MODULE
    pub player_projectiles: Vec::<shot::Shot>,
    pub player: entity::Entity,
    pub enemies: Vec::<entity::Entity>,


    // STUFF WE NEED
    pub controls: controls::Controls,
    pub scene: scene::Scene,
    pub level: level::Level,
    pub render_context: render_gl::context::Context,
    pub camera: camera::Camera,

    delta_time: deltatime::Deltatime,

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

        let delta_time = deltatime::Deltatime::new();


        Ok(Context {
            player_projectiles: Vec::<shot::Shot>::new(),
            enemies: vec! [enemy1],
            player,
            scene,
            controls,
            render_context,
            level,
            camera,
            delta_time
        })
    }

    pub fn update_game_state(&mut self) {

        self.delta_time.update();

        let delta = self.delta_time.millis();


        if self.player_projectiles.len() > 0 {

            println!("{}", self.player_projectiles.len());
        }




        for p in &mut self.player_projectiles {
            p.update(delta);
        }


        match self.controls.shoot_dir {
            Some(dir) =>
            {
                //todo check cooldown/shooting speed

                // spawn projectile with dir

                let mut found = false;
                'find: for p in &mut self.player_projectiles {
                    if p.expired {
                        found = true;
                        p.entity.velocity = dir;
                        p.entity.pos = self.player.pos;
                        p.time_remaining = 300;
                        p.expired = false;
                        break 'find;
                    }
                }

                if ! found {

                    let clr = na::Vector3::new(0.0,  0.0, 0.0);
                    let cube = cube::Cube::new(&self.render_context.res, clr, &self.render_context.gl).unwrap();
                    let mut projectile_e = entity::Entity::new(cube, self.player.pos);
                    projectile_e.velocity = dir;
                    let projectile = shot::Shot::new(projectile_e, 300);

                    self.player_projectiles.push(projectile);
                }

            }
            _ => {}
        }

        //println!("AFTER");

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

        for p in &self.player_projectiles {
            p.render(&self.render_context.gl, self.camera.projection(), self.camera.view());
        }

        self.render_context.window.gl_swap_window();
    }

}
