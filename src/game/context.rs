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

impl Cameras {

    pub fn current(&self) -> &dyn camera::Camera {
        match self.mode {
            camera::CameraMode::Free =>
                &self.free_camera,
            camera::CameraMode::Follow =>
                &self.follow_camera,
        }
    }
}


pub struct Context {

    // STUFF WE NEED
    pub controls: controls::Controls,
    pub scene: scene::Scene,
    pub level: level::Level,
    pub render_context: render_gl::context::Context,


    // CAMERAS
    pub cameras: Cameras,

    pub entities: entity::Entities,


    // MEHES AND SHADERS
    pub cube_shader: render_gl::Shader,
    pub mesh_shader: render_gl::Shader,
    pub hitbox_shader: render_gl::Shader,

    // make this a struct that can keep track of it, with usize ids, but not as a vec index
    // but something where we can add and remove from
    pub models: std::collections::HashMap<String, entity::Model>,

    pub actions: action_system::ActionsImpl,


    pub render_hitboxes: bool,

    delta_time: deltatime::Deltatime,

}

impl Context {

    pub fn new() -> Result<Context, failure::Error> {

        let mut ctx = empty()?;

        ctx.setup_player()?;

        ctx.setup_enemy()?;

        Ok(ctx)
    }



    fn setup_enemy(&mut self) -> Result<(), failure::Error>  {
        let player_glb_path = "E:/repos/Game-in-rust/blender_models/player.glb";

        let (skeleton, index_map) = render_gl::Skeleton::from_gltf(&player_glb_path)?;

        let gltf_meshes = render_gl::meshes_from_gltf(&player_glb_path, &self.render_context.gl, &index_map)?;

        let dummy_id = self.setup_hitbox_model("targetDummy", &gltf_meshes);

        self.entities.dummy_id = dummy_id;

        Ok(())

    }

    fn setup_player(&mut self) -> Result<(), failure::Error>  {

        let player_glb_path = "E:/repos/Game-in-rust/blender_models/player.glb";

        let (skeleton, index_map) = render_gl::Skeleton::from_gltf(&player_glb_path)?;

        let animations = load_animations(&player_glb_path, &skeleton).unwrap();

        let mut animation_player = render_gl::AnimationPlayer::new(render_gl::PlayerAnimation::Idle, &skeleton, animations);
        let gltf_meshes = render_gl::meshes_from_gltf(&player_glb_path, &self.render_context.gl, &index_map)?;

        let mut bones = Vec::new();
        let joint_count = skeleton.joints.len();
        for _ in 0..joint_count {
            bones.push(na::Matrix4::identity());
        }

        // MODELS

        let model_name = "player";
        self.add_skinned_model(model_name, &gltf_meshes);

        // ENTITY PLAYER
        let id = self.entities.next_id;
        let physics = entity::Physics::new(id);

        let health = entity::Health::new(100.0);

        let mut player = entity::Entity::new(physics, health, Some(animation_player), model_name.to_string());

        player.skeleton = skeleton;

        player.bones = bones;

        self.entities.add(player);
        self.entities.player_id = id;

        let hammer_id = self.setup_hitbox_model("hammer", &gltf_meshes);

        self.entities.hammer_id = hammer_id;

        Ok(())
    }



    fn setup_hitbox_model(&mut self, name: &str, gltf_meshes: &render_gl::GltfMeshes) -> usize {

        let hitboxes = gltf_meshes.hitboxes(name);

        self.add_model(name, &gltf_meshes);

        // ENTITY WEAPON
        let id = self.entities.next_id;

        let mut entity = entity::create_weapon(id, name.to_string(), &hitboxes);

        for hb_kv in &hitboxes {
            self.add_model(&hb_kv.0, &gltf_meshes);
        }

        self.entities.add(entity);

        id

    }


    fn add_skinned_model(&mut self, name: &str, gltf_meshes: &render_gl::GltfMeshes) {
        let model_mesh = render_gl::SkinnedMesh::new(&self.render_context.gl, &gltf_meshes.meshes[name]);

        let model = entity::Model::skinned_model(model_mesh);

        let model_name = name.to_string();

        self.models.insert(name.to_string(), model);

    }


    fn add_model(&mut self, name: &str, gltf_meshes: &render_gl::GltfMeshes) {
        println!("Add Model: {:#?}", name);
        let model_mesh = render_gl::Mesh::new(&self.render_context.gl, &gltf_meshes.meshes[name]);

        let model = entity::Model::mesh(model_mesh);

        let model_name = name.to_string();

        self.models.insert(name.to_string(), model);
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
        for entity in self.entities.values_mut() {
            entity.update_animations(delta);
        }

    }

    pub fn render(&mut self) {

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

        for entity in self.entities.values() {
            let model = &self.models[&entity.model_name];
            render_gl::render_entity(&entity, &self.entities, model, &self.render_context.gl, &self.mesh_shader);
        }


        if self.render_hitboxes {
            // TODO should be for all entities, but for now just weapon (hammeer)

            self.hitbox_shader.set_used();
            self.hitbox_shader.set_projection_and_view(&self.render_context.gl, self.camera().projection(), self.camera().view());
            let switched = false;

            if !self.render_context.wire_frame {
                self.render_context.switch_mode();
            }


            for entity in self.entities.values() {
                for hitbox in &entity.hit_boxes {
                    let model = &self.models[&hitbox.name];
                    render_gl::render_entity(&entity, &self.entities, model, &self.render_context.gl, &self.hitbox_shader);

                }
            }

            if !switched {
                self.render_context.switch_mode();
            }

        }

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

    let mesh_shader = render_gl::Shader::new("mesh_shader", &render_context.res, &render_context.gl)?;

    let hitbox_shader = render_gl::Shader::new("hitbox_shader", &render_context.res, &render_context.gl)?;



    let mut scene = scene::Scene::new(&level, &render_context)?;

    scene.add_box(na::Vector3::new(3.0, 0.0, 0.5));

    let controls = controls::Controls::new(event_pump);

    let delta_time = deltatime::Deltatime::new();

    let actions = action_system::load_player_actions(&render_context.res)?;





    let cameras = Cameras {
        free_camera,
        follow_camera,
        mode: camera::CameraMode::Follow
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
        hitbox_shader,
        entities,
        cameras,
        models: std::collections::HashMap::new(),
        render_hitboxes: false,
    })
}

fn load_animations(file_path: &str, skeleton: &render_gl::Skeleton) -> Option<render_gl::PlayerAnimations>{

    let animations = match render_gl::load_animations(file_path, &skeleton) {
        Ok(key_frames) => key_frames,
        Err(err) => {           //
            println!("Error loading key_frames: {:#?}", err);
            return None; }
    };

    Some(animations)
}
