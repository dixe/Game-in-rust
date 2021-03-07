use nalgebra as na;

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

    pub enemies: Vec<usize>,
    pub player_id: usize,

    // STUFF WE NEED
    pub controls: controls::Controls,
    pub scene: scene::Scene,
    pub level: level::Level,
    pub render_context: render_gl::context::Context,
    pub camera: camera::Camera,

    pub entity_manager: entity::EntityManager,

    pub player_projectile_model_id: usize,
    pub enemy_model_id: usize,

    delta_time: deltatime::Deltatime,

}

impl Context {



    pub fn new() -> Result<Context, failure::Error> {

        let mut ctx = empty()?;

        ctx.setup_player()?;

        ctx.setup_enemy_models()?;

        ctx.add_enemy();

        Ok(ctx)
    }


    fn setup_enemy_models(&mut self) -> Result<(), failure::Error> {

        let enemy_color = na::Vector3::new(0.3, 0.0, 0.0);

        let enemy_cube = cube::Cube::new(&self.render_context.res, enemy_color, &self.render_context.gl)?;

        let e_model = entity::Model::new(enemy_cube);

        self.enemy_model_id = self.entity_manager.add_model(e_model);

        Ok(())

    }


    fn setup_player(&mut self) -> Result<(), failure::Error>  {

        // PLAYER
        let player_pos = na::Vector3::new(3.0, 3.0, 0.0);
        let player_color = na::Vector3::new(0.0,  1.0, 1.0);

        let player_cube = cube::Cube::new(&self.render_context.res, player_color, &self.render_context.gl)?;

        let player_model = entity::Model::new(player_cube);

        let player_model_id = self.entity_manager.add_model(player_model);

        let player_id = self.entity_manager.add_entity(player_model_id, player_pos);

        self.player_id = player_id;

        let health = entity::Health::new(100.0);
        self.entity_manager.set_entity_health(player_id, health);


        // PLAYER PROJECTILE

        let player_projectile_color = na::Vector3::new(0.2,  1.0, 0.2);

        let player_projectile_cube = cube::Cube::new(&self.render_context.res, player_projectile_color, &self.render_context.gl)?;

        let mut proj_model = entity::Model::new(player_projectile_cube);

        let scale = &na::Vector3::new(0.3,0.3,0.3);
        proj_model.scale(&scale);

        let player_projectile_model_id = self.entity_manager.add_model(proj_model);

        self.player_projectile_model_id = player_projectile_model_id;

        Ok(())
    }

    fn add_enemy(&mut self) {

        // ENEMY
        let enemy_pos = na::Vector3::new(-3.0, -3.0, 0.0);

        let enemy_id = self.entity_manager.add_entity(self.enemy_model_id, enemy_pos);

        self.enemies.push(enemy_id);

        let health = entity::Health::new(100.0);

        self.entity_manager.set_entity_health(enemy_id, health);

        match self.entity_manager.get_entity(enemy_id) {
            Some(mut e) => {
                e.max_speed = 8.0;
                self.entity_manager.update_entity(enemy_id, e);
            },
            None => {}
        };

    }




    // Call once pr update step
    pub fn update_delta(&mut self) {
        self.delta_time.update();
    }

    pub fn get_delta_millis(&self) -> i32 {
        self.delta_time.millis()
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time.time()
    }


    pub fn handle_inputs(&mut self) {
        self.controls.handle_inputs(&mut self.render_context);
    }

    pub fn render(&self){


        self.scene.render(&self.render_context.gl, self.camera.projection(), self.camera.view());


        // player
        self.entity_manager.render(self.player_id, &self.render_context.gl, &self.camera.projection(), &self.camera.view());


        // enemies
        for id in &self.enemies {
            self.entity_manager.render(*id, &self.render_context.gl, &self.camera.projection(), &self.camera.view());
        }


        for p in &self.player_projectiles {
            self.entity_manager.render(p.entity_id, &self.render_context.gl, &self.camera.projection(), &self.camera.view());
        }

        self.render_context.window.gl_swap_window();
    }
}



fn empty() -> Result<Context, failure::Error> {

    let render_context = render_gl::context::setup()?;

    let background_color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));

    let entity_manager = entity::EntityManager::new();

    background_color_buffer.set_used(&render_context.gl);

    let camera = camera::Camera::new();

    let event_pump = render_context.sdl.event_pump().unwrap();

    let level = level::Level::load(&render_context.res,"levels/debugLevel1.txt")?;

    let scene = scene::Scene::new(&level, &render_context)?;

    let controls = controls::Controls::new(event_pump);

    let delta_time = deltatime::Deltatime::new();

    let enemies = Vec::new();


    Ok(Context {
        player_projectiles: Vec::<shot::Shot>::new(),
        player_id: 9999,
        scene,
        controls,
        render_context,
        level,
        camera,
        delta_time,
        entity_manager,
        enemies,
        player_projectile_model_id: 9999,
        enemy_model_id: 9999,
    })
}
