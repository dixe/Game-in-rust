use nalgebra as na;

use crate::cube;
use crate::entity;
use crate::render_gl;

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

    pub animations: std::collections::HashMap<String, render_gl::PlayerAnimations>,

    pub actions: action_system::ActionsImpl,


    pub render_hitboxes: bool,

    delta_time: deltatime::Deltatime,

}

impl Context {

    pub fn new() -> Result<Context, failure::Error> {

        let mut ctx = empty()?;

        println!("Setup player");
        ctx.setup_player()?;

        println!("Setup weapon");
        ctx.load_weapon()?;

        println!("Setup enemy");
        ctx.setup_enemy()?;

        Ok(ctx)
    }

    fn setup_enemy(&mut self) -> Result<(), failure::Error>  {
        let player_glb_path = "E:/repos/Game-in-rust/blender_models/player.glb";

        let (_skeleton, index_map) = render_gl::Skeleton::from_gltf(&player_glb_path)?;

        let gltf_meshes = render_gl::meshes_from_gltf(&player_glb_path, &self.render_context.gl, &index_map)?;

        let model_name = "targetDummy";
        self.add_model(model_name, &gltf_meshes);

        let mut dummy = entity::Entity::new(None, model_name.to_string());
        self.setup_hitboxes(&mut dummy, &gltf_meshes);
        dummy.physics.pos.x += 5.0;
        self.entities.enemies.add(dummy);

        Ok(())
    }


    fn load_weapon(&mut self) ->  Result<(), failure::Error>  {
        let glb_path = "E:/repos/Game-in-rust/blender_models/hammer.glb";

        let (skeleton, index_map) = render_gl::Skeleton::from_gltf(&glb_path)?;

        let _gltf_meshes = render_gl::meshes_from_gltf(&glb_path, &self.render_context.gl, &index_map)?;

        let animations = load_animations(&glb_path, &skeleton).unwrap();

        let gltf_meshes = render_gl::meshes_from_gltf(&glb_path, &self.render_context.gl, &index_map)?;
        let model_name = "hammer";

        self.add_model(model_name, &gltf_meshes);

        let mut weapon = entity::Entity::new(None, model_name.to_string());
        self.setup_hitboxes(&mut weapon, &gltf_meshes);

        self.animations.insert(model_name.to_string(), animations);
        self.entities.weapons.add(weapon);

        Ok(())
    }


    fn setup_player(&mut self) -> Result<(), failure::Error>  {

        let player_glb_path = "E:/repos/Game-in-rust/blender_models/player.glb";

        let (skeleton, index_map) = render_gl::Skeleton::from_gltf(&player_glb_path)?;

        let animations = load_animations(&player_glb_path, &skeleton).unwrap();

        self.animations.insert("player".to_string(), animations.clone());

        let animation_player = render_gl::AnimationPlayer::new(render_gl::Animation::Idle, animations);
        let gltf_meshes = render_gl::meshes_from_gltf(&player_glb_path, &self.render_context.gl, &index_map)?;

        let mut bones = Vec::new();
        let joint_count = skeleton.joints.len();
        for _ in 0..joint_count {
            bones.push(na::Matrix4::identity());
        }

        // MODELS
        let model_name = "player";
        self.add_skinned_model(model_name, &gltf_meshes);

        let mut player = entity::Entity::new(Some(animation_player), model_name.to_string());
        self.setup_hitboxes(&mut player, &gltf_meshes);

        player.skeleton = skeleton;
        player.bones = bones;

        self.entities.player = player;


        Ok(())
    }



    fn setup_hitboxes(&mut self, entity: &mut entity::Entity, gltf_meshes: &render_gl::GltfMeshes) {

        let hitboxes = gltf_meshes.hitboxes(&entity.model_name);

        let entity = entity::add_hitbox_to_entity(entity, &hitboxes);

        for hb_kv in &hitboxes {
            self.add_model(&hb_kv.0, &gltf_meshes);
        }
    }


    fn add_skinned_model(&mut self, name: &str, gltf_meshes: &render_gl::GltfMeshes) {
        let model_mesh = render_gl::SkinnedMesh::new(&self.render_context.gl, &gltf_meshes.meshes[name]);

        let model = entity::Model::skinned_model(model_mesh);

        let _model_name = name.to_string();

        self.models.insert(name.to_string(), model);

    }


    fn add_model(&mut self, name: &str, gltf_meshes: &render_gl::GltfMeshes) {
        //println!("Add Model: {:#?}", name);
        let model_mesh = render_gl::Mesh::new(&self.render_context.gl, &gltf_meshes.meshes[name]);

        let model = entity::Model::mesh(model_mesh);

        let _model_name = name.to_string();

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
        self.entities.player.update_animations(delta);

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
            self.render_hitboxes();
        }
    }


    fn render_hitboxes(&mut self) {

        //RENDER HITBOXES

        self.hitbox_shader.set_used();
        self.hitbox_shader.set_projection_and_view(&self.render_context.gl, self.camera().projection(), self.camera().view());
        let switched = false;

        if !self.render_context.wire_frame {
            self.render_context.switch_mode();
        }

        self.cube_shader.set_used();
        self.cube_shader.set_projection_and_view(&self.render_context.gl, self.camera().projection(), self.camera().view());
        for entity in self.entities.hitbox_entities() {
            match entity.is_hit {
                true => self.hitbox_shader.set_vec3(&self.render_context.gl, "color", na::Vector3::new(1.0, 0.0, 0.0)),
                false => self.hitbox_shader.set_vec3(&self.render_context.gl, "color", na::Vector3::new(1.0, 1.0, 1.0))
            };

            for hitbox in &entity.hit_boxes {
                let col_box = hitbox.make_transformed(entity.physics.pos, entity.physics.rotation);

                let clr = na::Vector3::new(1.0, 1.0, 0.0);
                let cube_model = cube::Cube::from_collision_box(col_box, clr, &self.render_context.gl);

                cube_model.render(&self.render_context.gl, &self.hitbox_shader, na::Matrix4::identity());

            }
        }

        if !switched {
            self.render_context.switch_mode();
        }

    }

    pub fn reload_shaders(&mut self) {

        let shaders = vec![ ("mesh_shader", &mut self.mesh_shader), ("hitbox_shader", &mut self.hitbox_shader)];

        for (name, shader) in shaders {
            match render_gl::Shader::new(name, &self.render_context.res, &self.render_context.gl) {
                Ok(new_shader) => {

                    println!("Reloaded {}", name);
                    *shader = new_shader;
                },
                Err(err) => {
                    println!("Error loading {}: {}", name, err);
                }
            };
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
        animations: std::collections::HashMap::new(),
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
