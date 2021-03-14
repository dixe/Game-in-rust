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

    pub ecs: entity::EntityComponentSystem,

    pub player_projectile_model_id: usize,
    pub enemy_model_id: usize,

    pub cube_shader: render_gl::Shader,
    pub light_shader: render_gl::Shader,

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

        let enemy_cube = cube::Cube::new(enemy_color, &self.render_context.gl);

        let e_model = entity::Model::new(enemy_cube);

        self.enemy_model_id = self.ecs.add_model(e_model);

        Ok(())

    }


    fn setup_player(&mut self) -> Result<(), failure::Error>  {
        let player_id = self.ecs.add_entity();

        let loaded_model = render_gl::Model::load_from_path(&self.render_context.gl, "models/sphere.obj", &self.render_context.res);


        // MODEL
        let player_color = na::Vector3::new(0.0, 1.0, 1.0);
        let player_cube = cube::Cube::new(player_color, &self.render_context.gl);
        let player_model = entity::Model::new(player_cube);
        let player_model_id = self.ecs.add_model(player_model);


        // SHOOTER
        let player_shooter = entity::Shooter::default();
        self.ecs.set_shooter(player_id, player_shooter);


        self.ecs.set_model(player_id, player_model_id);


        let mut physics = entity::Physics::new(player_id, player_model_id);
        physics.pos =  na::Vector3::new(0.0, 3.0, 0.5);


        self.ecs.set_physics(player_id, physics);

        self.player_id = player_id;

        let health = entity::Health::new(100.0);
        self.ecs.set_health(player_id, health);


        // PLAYER PROJECTILE

        let player_projectile_color = na::Vector3::new(0.2,  1.0, 0.2);

        let player_projectile_cube = cube::Cube::new(player_projectile_color, &self.render_context.gl);

        let mut proj_model = entity::Model::new(player_projectile_cube);

        let player_projectile_model_id = self.ecs.add_model(proj_model);

        self.player_projectile_model_id = player_projectile_model_id;

        Ok(())
    }

    fn add_enemy(&mut self) {

        // ENEMY
        let enemy_id = self.ecs.add_entity();

        self.enemies.push(enemy_id);

        let health = entity::Health::new(100.0);


        let mut physics = entity::Physics::new(enemy_id, self.enemy_model_id);
        physics.pos = na::Vector3::new(0.0, -3.0, 0.5);

        self.ecs.set_physics(enemy_id, physics);

        self.ecs.set_health(enemy_id, health);

        match self.ecs.get_physics(enemy_id) {
            Some(e) => {
                let mut phy = *e;
                phy.max_speed = 8.0;
                self.ecs.set_physics(enemy_id, phy);
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
        let action = self.controls.handle_inputs(&mut self.render_context);

        match action {
            controls::Action::AddEnemy => self.add_enemy(),
            controls::Action::NoAction => { },
        };
    }



    pub fn render(&self){
        // RENDER SCENE WITH CUBE SHADER
        self.cube_shader.set_used();
        // CAN BE MOVED OUTSIDE THE LOOP
        self.cube_shader.set_vec3(&self.render_context.gl, "lightPos", na::Vector3::new(0.0, 0.0, 5.0)); //
        self.cube_shader.set_vec3(&self.render_context.gl, "lightColor", na::Vector3::new(1.0, 1.0, 1.0));

        self.cube_shader.set_projection_and_view(&self.render_context.gl, self.camera.projection(), self.camera.view());

        self.scene.render(&self.render_context.gl, &self.cube_shader);


        // player
        self.ecs.render(self.player_id, &self.render_context.gl, &self.cube_shader);


        // enemies
        for id in &self.enemies {
            self.ecs.render(*id, &self.render_context.gl, &self.cube_shader);
        }


        for p in &self.player_projectiles {
            self.ecs.render(p.entity_id, &self.render_context.gl, &self.cube_shader);
        }


    }
}






fn empty() -> Result<Context, failure::Error> {

    let render_context = render_gl::context::setup()?;

    let background_color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));

    let ecs = entity::EntityComponentSystem::new();


    background_color_buffer.set_used(&render_context.gl);

    let camera = camera::Camera::new();

    let event_pump = render_context.sdl.event_pump().unwrap();

    let level = level::Level::load(&render_context.res,"levels/debugLevel1.txt")?;

    let cube_shader = render_gl::Shader::new("light_color_shader", &render_context.res, &render_context.gl)?;

    let light_shader = render_gl::Shader::new("lightcube", &render_context.res, &render_context.gl)?;

    let mut scene = scene::Scene::new(&level, &render_context)?;

    scene.add_box(na::Vector3::new(3.0, 0.0, 0.5));

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
        ecs,
        enemies,
        player_projectile_model_id: 9999,
        enemy_model_id: 9999,
        cube_shader,
        light_shader
    })
}
