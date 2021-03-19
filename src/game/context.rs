use nalgebra as na;

use crate::game;
use crate::entity;
use crate::render_gl;
use crate::cube;
use crate::camera;
use crate::scene;
use crate::level;
use crate::controls;
use crate::deltatime;
use crate::animation_system as anim_sys;
struct NewModel {
    entity_id: usize,
    model_id: usize,
}

pub struct Context {

    // should be in ecs
    //GAME STATE SHOULD MOVE INTO STRUCT/MODULE
    pub state: game::State,
    pub player_id: usize,

    // STUFF WE NEED
    pub controls: controls::Controls,
    pub scene: scene::Scene,
    pub level: level::Level,
    pub render_context: render_gl::context::Context,
    pub camera: camera::Camera,

    pub ecs: entity::EntityComponentSystem,

    pub projectile_model_id: usize,
    pub sword_model_id: usize,

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

        //ctx.add_enemy();

        Ok(ctx)
    }


    fn setup_enemy_models(&mut self) -> Result<(), failure::Error> {

        let enemy_color = na::Vector3::new(0.3, 0.0, 0.0);

        let enemy_cube = cube::Cube::new(enemy_color, &self.render_context.gl);

        let e_model = entity::Model::cube(enemy_cube);

        self.enemy_model_id = self.ecs.add_model(e_model);

        Ok(())

    }


    fn setup_player(&mut self) -> Result<(), failure::Error>  {
        let player_id = self.ecs.add_entity();

        // MODEL
        let player_color = na::Vector3::new(0.0, 1.0, 1.0);
        let player_cube = cube::Cube::new(player_color, &self.render_context.gl);


        // Use loaded model
        //let player_model = entity::Model::cube(player_cube);

        let loaded_model = render_gl::Model::load_from_path(&self.render_context.gl, player_color, "models/sphere.obj", &self.render_context.res)?;

        let player_model = entity::Model::wave_model(loaded_model);

        let player_model_id = self.ecs.add_model(player_model);


        // SHOOTER
        let player_shooter = entity::Shooter::default_player();
        self.ecs.set_shooter(player_id, player_shooter);
        self.ecs.set_model(player_id, player_model_id);

        let mut physics = entity::Physics::new(player_id, player_model_id);
        physics.pos = na::Vector3::new(0.0, 3.0, 0.5);


        self.ecs.set_physics(player_id, physics);

        self.player_id = player_id;

        let health = entity::Health::new(100.0);
        self.ecs.set_health(player_id, health);


        //PLAYER SWORD

        let sword = self.add_model_with_physics(na::Vector3::new(0.2, 0.2, 0.2), 3.0, "models/sword_01.obj")?;
        self.sword_model_id = sword.model_id;


        let complex = entity::ComplexEntity {
            id: player_id,
            sub_entities: vec![sword.entity_id],
        };

        self.ecs.set_entity_type(player_id, entity::EntityType::Complex(complex));

        // SWORD ANIMATION
        let sword_animation = entity::AnimationData::new(sword.entity_id, anim_sys::idle_bob_z);


        self.ecs.set_animation(sword.entity_id, sword_animation);
        // PLAYER PROJECTILE
        let player_projectile_color = na::Vector3::new(0.2,  1.0, 0.2);

        let player_projectile_cube = cube::Cube::new(player_projectile_color, &self.render_context.gl);

        let mut proj_model = entity::Model::cube(player_projectile_cube);

        let projectile_model_id = self.ecs.add_model(proj_model);

        self.projectile_model_id = projectile_model_id;



        println!("Plyaer id = {}", player_id);

        Ok(())
    }


    fn add_model_with_physics(&mut self, clr: na::Vector3::<f32>, scale: f32, model_path: &str) -> Result<NewModel, failure::Error>  {

        let model = render_gl::Model::load_from_path(&self.render_context.gl, clr, model_path, &self.render_context.res)?;
        let model_entity = entity::Model::wave_model(model);
        let model_id = self.ecs.add_model(model_entity);
        let entity_id = self.ecs.add_entity();

        let mut physics = entity::Physics::new(entity_id, model_id);
        physics.scale = scale;
        physics.pos.y -= 1.5;


        physics.rotation.x += 1.57;


        self.ecs.set_physics(entity_id, physics);



        Ok(NewModel {
            model_id,
            entity_id,
        })


    }

    fn add_enemy(&mut self) {
        // ENEMY
        let enemy_id = self.ecs.add_entity();

        self.state.enemies.insert(enemy_id);

        let health = entity::Health::new(100.0);

        let mut physics = entity::Physics::new(enemy_id, self.enemy_model_id);
        physics.pos = na::Vector3::new(0.0, -3.0, 0.5);


        // SHOOTER
        let shooter = entity::Shooter::default_enemy();

        self.ecs.set_shooter(enemy_id, shooter);

        self.ecs.set_physics(enemy_id, physics);

        self.ecs.set_health(enemy_id, health);

        match self.ecs.get_physics(enemy_id) {
            Some(e) => {
                let mut enemy_physics = *e;
                enemy_physics.max_speed = 8.0;
                self.ecs.set_physics(enemy_id, enemy_physics);
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


        // all in state
        self.state.render(&self.ecs, &self.render_context.gl, &self.cube_shader);


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


    Ok(Context {
        player_id: 9999,
        scene,
        controls,
        render_context,
        level,
        camera,
        delta_time,
        ecs,
        projectile_model_id: 9999,
        sword_model_id: 9999,
        enemy_model_id: 9999,
        cube_shader,
        light_shader,
        state: game::State::new(),
    })
}
