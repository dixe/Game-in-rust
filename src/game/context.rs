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
use crate::action_system;

struct NewModel {
    entity_id: usize,
    init_physics: entity::Physics,
}

pub struct Cameras {
    free_camera: camera::FreeCamera,
    follow_camera: camera::FollowCamera,
    pub mode: camera::CameraMode,
}


pub struct Context {

    // should be in ecs
    //GAME STATE SHOULD MOVE INTO STRUCT/MODULE
    pub state: game::State,
    pub player_id: usize,
    pub player_weapon_id: usize,


    // STUFF WE NEED
    pub controls: controls::Controls,
    pub scene: scene::Scene,
    pub level: level::Level,
    pub render_context: render_gl::context::Context,


    // CAMERAS
    cameras: Cameras,


    pub ecs: entity::EntityComponentSystem,

    pub projectile_model_id: usize,

    pub enemy_model_id: usize,

    pub cube_shader: render_gl::Shader,

    pub swing_animation: Option<render_gl::Animation>,

    pub actions: action_system::ActionsImpl,



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

        println!("Enemy model id: {}", self.enemy_model_id);
        Ok(())

    }



    pub fn load_model(&mut self, entity_id: usize,  clr: na::Vector3::<f32>, model_path: &str) ->  Result<(), failure::Error> {

        // RELOADING MODELS WITH THIS WILL LEAK THE DATA, SINCE IT WILL ONLY ADD TO MODELS VEC
        // BUT NOT REMOVE THE OLD UNUSED ONES. FINE FOR DEBUGGING
        let (model, _anchors) = render_gl::Model::load_from_path_tobj(&self.render_context.gl, clr, model_path, &self.render_context.res)?;

        let model_entity = entity::Model::wave_model(model);
        let model_id = self.ecs.add_model(model_entity);

        self.ecs.set_model(entity_id, model_id);
        Ok(())
    }




    fn setup_player(&mut self) -> Result<(), failure::Error>  {
        let player_id = self.ecs.add_entity();

        // MODEL
        let player_color = na::Vector3::new(0.0, 1.0, 1.0);

        let (loaded_model, weapon_anchor) = render_gl::Model::load_from_path_tobj(&self.render_context.gl, player_color, "models/player.obj", &self.render_context.res)?;



        let player_model = entity::Model::wave_model(loaded_model);
        let player_model_id = self.ecs.add_model(player_model);
        self.ecs.set_model(player_id, player_model_id);


        // SHOOTER
        let player_shooter = entity::Shooter::default_player();
        self.ecs.set_shooter(player_id, player_shooter);


        // PHYSICS
        let mut physics = entity::Physics::new(player_id);
        physics.pos.x -= 2.0;

        self.ecs.set_physics(player_id, physics);

        self.player_id = player_id;

        let health = entity::Health::new(100.0);
        self.ecs.set_health(player_id, health);



        //SWORD
        let sword = self.add_model_with_physics(na::Vector3::new(0.2, 0.2, 0.2), 1.0, Some(player_id), "models/sword.obj")?;


        weapon_anchor.map(|anchor| {
            self.ecs.set_anchor_point(sword.entity_id, anchor);
            match self.swing_animation  {
                Some(ref ani) => {
                    self.actions.swing = action_system::from_anchor_points(&ani.frame_anchors, anchor );
                }, _ => {}
            };
        });



        self.player_weapon_id = sword.entity_id;

        // SWORD ACTION
        let _sword_idle = entity::ActionData::new(action_system::Actions::Idle, None, sword.init_physics);
        let actions_info = entity::ActionsInfo::new(sword.entity_id, None);

        self.ecs.set_actions_info(sword.entity_id, actions_info);


        // PLAYER PROJECTILE
        let player_projectile_color = na::Vector3::new(0.2,  1.0, 0.2);

        let player_projectile_cube = cube::Cube::new(player_projectile_color, &self.render_context.gl);

        let proj_model = entity::Model::cube(player_projectile_cube);

        let projectile_model_id = self.ecs.add_model(proj_model);

        self.projectile_model_id = projectile_model_id;

        println!("Player id = {}", player_id);
        println!("Player_weapon id = {}", self.player_weapon_id);

        Ok(())
    }




    fn add_model_with_physics(&mut self, clr: na::Vector3::<f32>, scale: f32, anchor_id: Option<usize>, model_path: &str) -> Result<NewModel, failure::Error>  {


        let (model, _anchors) = render_gl::Model::load_from_path_tobj(&self.render_context.gl, clr, model_path, &self.render_context.res)?;
        let model_entity = entity::Model::wave_model(model);
        let model_id = self.ecs.add_model(model_entity);
        let entity_id = self.ecs.add_entity();

        self.ecs.set_model(entity_id, model_id);

        let mut physics = entity::Physics::new(entity_id);
        physics.scale = scale;
        physics.inverse_mass = 0.0;

        physics.anchor_id = anchor_id;

        self.ecs.set_physics(entity_id, physics);

        Ok(NewModel {
            entity_id,
            init_physics: physics,
        })


    }

    fn add_enemy(&mut self) {
        // ENEMY
        let enemy_id = self.ecs.add_entity();

        self.state.enemies.insert(enemy_id);

        let health = entity::Health::new(100.0);

        let mut physics = entity::Physics::new(enemy_id);
        physics.pos = na::Vector3::new(0.0, -3.0, 4.5);

        self.ecs.set_model(enemy_id, self.enemy_model_id);

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


    pub fn camera(&self) -> &dyn camera::Camera {
        match self.cameras.mode {
            camera::CameraMode::Free =>
                &self.cameras.free_camera,
            camera::CameraMode::Follow =>
                &self.cameras.follow_camera,
        }
    }


    pub fn camera_mut(&mut self) -> &mut dyn camera::Camera {
        match self.cameras.mode {
            camera::CameraMode::Free =>
                &mut self.cameras.free_camera,
            camera::CameraMode::Follow =>
                &mut self.cameras.follow_camera,
        }
    }


    pub fn reload_actions (&mut self) {
        let actions = action_system::load_player_actions(&self.render_context.res);
        match actions {
            Ok(act) => self.actions = act,
            Err(err) => println!("Reload actions error: {:#?}", err),
        }
    }


    // Call once pr update step
    pub fn update_delta(&mut self) {
        self.delta_time.update();
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time.time()
    }


    pub fn handle_inputs(&mut self) {
        let action = self.controls.handle_inputs(&mut self.render_context, &mut self.cameras);

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

        self.cube_shader.set_projection_and_view(&self.render_context.gl, self.camera().projection(), self.camera().view());

        self.scene.render(&self.render_context.gl, &self.cube_shader);


        // player
        match (&self.state.player_state, &self.swing_animation) {
            (game::PlayerState::Attacking, Some(ref swing)) => {
                // get player action and how far we are in it

                let _info = self.ecs.get_actions_info(self.player_weapon_id);

                let percent = self.ecs.get_actions_info(self.player_weapon_id).and_then(|info| info.active.map(|a| a.percent_done())).unwrap_or_default();

                render_gl::render(&self.ecs, self.player_id, &self.render_context.gl, &self.cube_shader, Some((&swing, percent)));
            },
            _ => {
                render_gl::render(&self.ecs, self.player_id, &self.render_context.gl, &self.cube_shader, None);
            }
        };

        render_gl::render(&self.ecs, self.player_weapon_id, &self.render_context.gl, &self.cube_shader, None);


        // all in state
        self.state.render(&self.ecs, &self.render_context.gl, &self.cube_shader);

    }

}




fn empty() -> Result<Context, failure::Error> {

    let width = 900;
    let height = 700;

    let render_context = render_gl::context::setup(width, height)?;

    let background_color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));

    let ecs = entity::EntityComponentSystem::new();

    background_color_buffer.set_used(&render_context.gl);

    let follow_camera = camera::FollowCamera::new(width, height); //
    let free_camera = camera::FreeCamera::new();




    let event_pump = render_context.sdl.event_pump().unwrap();

    let level = level::Level::load(&render_context.res,"levels/debugLevel1.txt")?;

    let cube_shader = render_gl::Shader::new("light_color_shader", &render_context.res, &render_context.gl)?;

    let mut scene = scene::Scene::new(&level, &render_context)?;

    scene.add_box(na::Vector3::new(3.0, 0.0, 0.5));

    let controls = controls::Controls::new(event_pump);

    let delta_time = deltatime::Deltatime::new();

    let actions = action_system::load_player_actions(&render_context.res)?;

    let _player_color = na::Vector3::new(0.0, 1.0, 1.0);
    let swing_animation = None;
    //swing_animation = Some(render_gl::Animation::load_from_path(&render_context.gl, player_color, "animations/slap/", &render_context.res)?);


    let cameras = Cameras {
        free_camera,
        follow_camera,
        mode: camera::CameraMode::Free
    };



    Ok(Context {
        player_id: 9999,
        scene,
        controls,
        render_context,
        level,
        delta_time,
        ecs,
        actions,
        projectile_model_id: 9999,
        enemy_model_id: 9999,
        cube_shader,
        state: game::State::new(),
        player_weapon_id: 9999,
        swing_animation,
        cameras,
    })
}
