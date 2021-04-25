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


pub struct Cameras {
    free_camera: camera::FreeCamera,
    follow_camera: camera::FollowCamera,
    pub mode: camera::CameraMode,
}


pub struct Context {

    // STUFF WE NEED
    pub controls: controls::Controls,
    pub scene: scene::Scene,
    pub level: level::Level,
    pub render_context: render_gl::context::Context,


    // CAMERAS
    cameras: Cameras,

    pub entities: entity::Entities,


    pub cube_shader: render_gl::Shader,
    pub mesh_shader: render_gl::Shader,


    pub actions: action_system::ActionsImpl,

    pub models: Vec::<entity::Model>,


    delta_time: deltatime::Deltatime,

}

impl Context {

    pub fn new() -> Result<Context, failure::Error> {

        let mut ctx = empty()?;

        ctx.setup_player()?;

        //ctx.setup_enemy_models()?;

        //ctx.add_enemy();

        Ok(ctx)
    }

    /*
    fn setup_enemy_models(&mut self) -> Result<(), failure::Error> {

    let enemy_color = na::Vector3::new(0.3, 0.0, 0.0);

    let enemy_cube = cube::Cube::new(enemy_color, &self.render_context.gl);

    let e_model = entity::Model::cube(enemy_cube);

    self.enemy_model_id = self.ecs.add_model(e_model);

    println!("Enemy model id: {}", self.enemy_model_id);
    Ok(())

}
     */


    fn setup_player(&mut self) -> Result<(), failure::Error>  {

        let (skeleton, index_map) = render_gl::Skeleton::from_gltf()?;

        let player_animations = load_player_animations(&skeleton).unwrap();

        let mut animation_player = render_gl::AnimationPlayer::new(render_gl::PlayerAnimation::Walk, player_animations);
        let mesh = render_gl::SkinnedMesh::from_gltf(&self.render_context.gl, &index_map)?;


        let mut bones = Vec::new();
        let joint_count = skeleton.joints.len();
        for _ in 0..joint_count {
            bones.push(na::Matrix4::identity());
        }
        animation_player.set_bones(bones);

        let player_model = entity::Model::skinned_model(mesh);

        self.models.push(player_model);

        let model_id = self.models.len() - 1;
        let id = self.entities.next_id;
        let physics = entity::Physics::new(id);


        let health = entity::Health::new(100.0);

        let player = entity::Entity {
            physics,
            animation_player,
            health,
            model_id
        };

        self.entities.add(player);
        self.entities.player_id = id;


        Ok(())
    }


    /*
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

    self.player_weapon_id = sword.entity_id;

    // SWORD ACTION

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

     */



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
            controls::Action::AddEnemy => {},
            controls::Action::NoAction => { },
        };
    }


    pub fn update_animations(&mut self) {
        let delta = self.get_delta_time();

        let mut animation_player = &mut self.entities.player_mut().animation_player;

        animation_player.set_frame_bones(delta);


    }




    pub fn render(&self){


        // RENDER SCENE WITH CUBE SHADER
        self.cube_shader.set_used();

        // CAN BE MOVED OUTSIDE THE LOOP
        self.cube_shader.set_vec3(&self.render_context.gl, "lightPos", na::Vector3::new(0.0, 0.0, 5.0));
        self.cube_shader.set_vec3(&self.render_context.gl, "lightColor", na::Vector3::new(1.0, 1.0, 1.0));

        self.cube_shader.set_projection_and_view(&self.render_context.gl, self.camera().projection(), self.camera().view());

        self.scene.render(&self.render_context.gl, &self.cube_shader);


        // RENDER WITH MESH SHADER

        self.mesh_shader.set_used();

        self.mesh_shader.set_vec3(&self.render_context.gl, "lightPos", na::Vector3::new(1.0, 0.0, 7.0));
        self.mesh_shader.set_vec3(&self.render_context.gl, "lightColor", na::Vector3::new(1.0, 1.0, 1.0));

        self.mesh_shader.set_projection_and_view(&self.render_context.gl, self.camera().projection(), self.camera().view());


        let player = self.entities.player();

        let player_model = &self.models[player.model_id];

        render_gl::render_entity(&player, &self.entities, player_model, &self.render_context.gl, &self.mesh_shader);

    }
}




fn empty() -> Result<Context, failure::Error> {

    let width = 900;
    let height = 700;

    let render_context = render_gl::context::setup(width, height)?;

    let background_color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));

    let entities = entity::Entities::new();

    background_color_buffer.set_used(&render_context.gl);

    let follow_camera = camera::FollowCamera::new(width, height); //
    let free_camera = camera::FreeCamera::new();




    let event_pump = render_context.sdl.event_pump().unwrap();

    let level = level::Level::load(&render_context.res,"levels/debugLevel1.txt")?;



    let cube_shader = render_gl::Shader::new("light_color_shader", &render_context.res, &render_context.gl)?;
    let mut mesh_shader = render_gl::Shader::new("mesh_shader", &render_context.res, &render_context.gl)?;



    let mut scene = scene::Scene::new(&level, &render_context)?;

    scene.add_box(na::Vector3::new(3.0, 0.0, 0.5));

    let controls = controls::Controls::new(event_pump);

    let delta_time = deltatime::Deltatime::new();

    let actions = action_system::load_player_actions(&render_context.res)?;





    let cameras = Cameras {
        free_camera,
        follow_camera,
        mode: camera::CameraMode::Free
    };



    Ok(Context {
        scene,
        controls,
        render_context,
        mesh_shader,
        level,
        delta_time,
        actions,
        cube_shader,
        entities,
        cameras,
        models: Vec::new(),
    })
}

fn load_player_animations(skeleton: &render_gl::Skeleton) -> Option<render_gl::PlayerAnimations>{

    let player_animations = match render_gl::load_player_animations(&skeleton) {
        Ok(key_frames) => key_frames,
        Err(err) => {           //
            println!("Error loading key_frames: {:#?}", err);
            return None; }
    };

    Some(player_animations)
}
