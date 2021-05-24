use nalgebra as na;


use crate::physics;
use crate::cube;
use crate::entity;
use crate::render_gl;

use crate::camera;
use crate::controls;
use crate::action_system;
use crate::game::ai;

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


pub struct Scene {

    // CAMERAS
    pub cameras: Cameras,

    pub entities: entity::Entities,

    // MEHES AND SHADERS
    pub cube_shader: render_gl::Shader,
    pub mesh_shader: render_gl::Shader,
    pub hitbox_shader: render_gl::Shader,


    // World
    pub world_triangles: Vec::<physics::Triangle>,

    // make this a struct that can keep track of it, with usize ids, but not as a vec index
    // but something where we can add and remove from
    pub models: std::collections::HashMap<String, entity::Model>,

    pub animations: std::collections::HashMap<String, render_gl::PlayerAnimations>,

    pub actions: action_system::ActionsImpl,

    pub render_hitboxes: bool,

    pub ais: std::collections::HashMap<usize, ai::Ai>,

}

impl Scene {

    pub fn new(render_context: &render_gl::context::Context) -> Result<Scene, failure::Error> {

        let mut scene = empty(render_context)?;

        println!("Setup world");
        scene.setup_world(&render_context.gl)?;

        println!("Setup player");
        scene.setup_player(&render_context.gl)?;

        println!("Setup weapon");
        scene.load_weapon(&render_context.gl)?;

        println!("Setup enemy");
        //scene.setup_enemy(&render_context.gl)?;

        Ok(scene)
    }

    fn setup_enemy(&mut self, gl: &gl::Gl) -> Result<(), failure::Error>  {

        let enemy_glb_path = "E:/repos/Game-in-rust/blender_models/enemy1.glb";

        let (skeleton, index_map) = render_gl::Skeleton::from_gltf(&enemy_glb_path)?;

        let animations = load_animations(&enemy_glb_path, &skeleton, None).unwrap();

        self.animations.insert("enemy1".to_string(), animations.clone());

        let animation_player = render_gl::AnimationPlayer::new(render_gl::Animation::Idle, animations);
        let gltf_meshes = render_gl::meshes_from_gltf(&enemy_glb_path, gl, &index_map)?;

        let mut bones = Vec::new();
        let joint_count = skeleton.joints.len();
        for _ in 0..joint_count {
            bones.push(na::Matrix4::identity());
        }

        // MODELS
        let model_name = "enemy";
        self.add_skinned_model(gl, model_name, &gltf_meshes);

        let mut enemy = entity::Entity::new(Some(animation_player), model_name.to_string());
        self.setup_hitboxes(gl, &mut enemy, &gltf_meshes);

        enemy.skeleton = skeleton;
        enemy.bones = bones;

        enemy.physics.pos.x += 5.0;

        enemy.queued_action = Some(entity::EntityState::Idle);
        enemy.next_action();
        let id = self.entities.enemies.add(enemy);

        self.ais.insert(id, ai::Ai::idle());

        Ok(())
    }


    fn load_weapon(&mut self, gl: &gl::Gl) ->  Result<(), failure::Error>  {
        let glb_path = "E:/repos/Game-in-rust/blender_models/hammer.glb";

        let (skeleton, index_map) = render_gl::Skeleton::from_gltf(&glb_path)?;
        let base_animations = Some(&self.entities.player.animation_player.as_ref().unwrap().animations);
        let animations = load_animations(&glb_path, &skeleton, base_animations).unwrap();

        let gltf_meshes = render_gl::meshes_from_gltf(&glb_path, gl, &index_map)?;
        let model_name = "hammer";

        self.add_model(gl, model_name, &gltf_meshes);

        let mut weapon = entity::Entity::new(None, model_name.to_string());
        self.setup_hitboxes(gl, &mut weapon, &gltf_meshes);

        self.animations.insert(model_name.to_string(), animations);
        self.entities.weapons.add(weapon);



        Ok(())
    }



    fn setup_world(&mut self, gl: &gl::Gl) -> Result<(), failure::Error>  {

        let world_glb_path = "E:/repos/Game-in-rust/blender_models/world_1.glb";

        let index_map = std::collections::HashMap::new();

        let gltf_meshes = render_gl::meshes_from_gltf(&world_glb_path, gl, &index_map)?;

        let model_name = "world";

        self.world_triangles = gltf_meshes.triangles(model_name);

        self.add_model(gl, model_name, &gltf_meshes);

        Ok(())


    }

    fn setup_player(&mut self, gl: &gl::Gl) -> Result<(), failure::Error>  {

        let player_glb_path = "E:/repos/Game-in-rust/blender_models/player.glb";

        let (skeleton, index_map) = render_gl::Skeleton::from_gltf(&player_glb_path)?;

        let animations = load_animations(&player_glb_path, &skeleton, None).unwrap();

        self.animations.insert("player".to_string(), animations.clone());

        let animation_player = render_gl::AnimationPlayer::new(render_gl::Animation::Idle, animations);
        let gltf_meshes = render_gl::meshes_from_gltf(&player_glb_path, gl, &index_map)?;

        let mut bones = Vec::new();
        let joint_count = skeleton.joints.len();
        for _ in 0..joint_count {
            bones.push(na::Matrix4::identity());
        }

        // MODELS
        let model_name = "player";
        self.add_skinned_model(gl, model_name, &gltf_meshes);

        let mut player = entity::Entity::new(Some(animation_player), model_name.to_string());
        self.setup_hitboxes(gl, &mut player, &gltf_meshes);

        player.skeleton = skeleton;
        player.bones = bones;


        for hitbox in &player.hitboxes {
            println!("{:?}",  hitbox.name);
        }

        self.entities.player = player;


        Ok(())
    }



    fn setup_hitboxes(&mut self, gl: &gl::Gl, entity: &mut entity::Entity, gltf_meshes: &render_gl::GltfMeshes) {

        let hitboxes = gltf_meshes.hitboxes(&entity.model_name);

        let entity = entity::add_hitbox_to_entity(entity, &hitboxes);

        for hb_kv in &hitboxes {
            self.add_model(gl, &hb_kv.0, &gltf_meshes);
        }
    }


    fn add_skinned_model(&mut self, gl: &gl::Gl, name: &str, gltf_meshes: &render_gl::GltfMeshes) {
        let model_mesh = render_gl::SkinnedMesh::new(gl, &gltf_meshes.meshes[name]);

        let model = entity::Model::skinned_model(model_mesh);

        let _model_name = name.to_string();

        self.models.insert(name.to_string(), model);

    }


    fn add_model(&mut self, gl: &gl::Gl, name: &str, gltf_meshes: &render_gl::GltfMeshes) {
        //println!("Add Model: {:#?}", name);
        let model_mesh = render_gl::Mesh::new(gl, &gltf_meshes.meshes[name]);

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


    pub fn update_animations(&mut self, delta: f32) {

        self.entities.player.update_animations(delta);
        for enemy in self.entities.enemies.values_mut() {
            enemy.update_animations(delta);
        }

    }

    pub fn render(&mut self, render_context: &mut render_gl::context::Context) {
        let gl = &render_context.gl;
        // RENDER SCENE WITH CUBE SHADER
        self.cube_shader.set_used();

        // CAN BE MOVED OUTSIDE THE LOOP
        self.cube_shader.set_vec3(gl, "lightPos", na::Vector3::new(0.0, 0.0, 5.0));
        self.cube_shader.set_vec3(gl, "lightColor", na::Vector3::new(1.0, 1.0, 1.0));

        self.cube_shader.set_projection_and_view(gl, self.camera().projection(), self.camera().view());

        // RENDER WITH MESH SHADER

        self.mesh_shader.set_used();
        self.mesh_shader.set_vec3(gl, "lightPos", na::Vector3::new(1.0, 0.0, 7.0));
        self.mesh_shader.set_vec3(gl, "lightColor", na::Vector3::new(1.0, 1.0, 1.0));
        self.mesh_shader.set_projection_and_view(gl, self.camera().projection(), self.camera().view());

        let model = &self.models["world"];
        render_gl::render_world(model, gl, &self.mesh_shader);

        for entity in self.entities.values() {
            let model = &self.models[&entity.model_name];
            render_gl::render_entity(&entity, model, gl, &self.mesh_shader);
        }


        if self.render_hitboxes {
            self.render_hitboxes(render_context);
        }

        self.render_ik_targets(render_context);
    }


    fn render_ik_targets(&mut self, render_context: &mut render_gl::context::Context) {

        let skeleton = &self.entities.player.skeleton;
        let gl = &render_context.gl;

        let ik = &skeleton.left_leg.as_ref().unwrap();


        let clr = na::Vector3::new(1.0, 0.0, 0.0);
        let cube_model = cube::Cube::new(clr, gl);

        let mut scale_mat = na::Matrix4::identity();


        scale_mat = scale_mat * 0.2;
        scale_mat[15] = 1.0;

        self.cube_shader.set_used();
        let proj = self.camera().projection();
        let view = self.camera().view();
        self.cube_shader.set_projection_and_view(gl, proj, view);

        // maybe do this and translation to the ik.target.translation and rotation
        // so render is just ik.target.translation for tran, and . rotation for rot
        // As it is now we might forget go get it in correct spot at some point
        // on the other hand no, we need to store base target anyways

        let rot_mat = self.entities.player.physics.rotation.to_homogeneous();
        let trans_mat = na::Matrix4::new_translation(&ik.current_target.translation);

        cube_model.render(gl, &self.cube_shader, trans_mat * rot_mat * scale_mat);

        self.render_pos(render_context, &(ik.joint_pos(0, &skeleton.joints) + self.entities.player.physics.pos));

        self.render_pos(render_context, &(ik.joint_pos(1, &skeleton.joints) + self.entities.player.physics.pos));

        self.render_pos(render_context, &(ik.joint_pos(2, &skeleton.joints) + self.entities.player.physics.pos));

        //self.render_pos(render_context, &ik.target.translation);

    }

    fn render_pos(&self, render_context: &mut render_gl::context::Context, pos: &na::Vector3::<f32>) {

        let gl = &render_context.gl;

        let clr = na::Vector3::new(0.0, 1.0, 0.0);
        let cube_model = cube::Cube::new(clr, gl);

        let mut scale_mat = na::Matrix4::identity();


        scale_mat = scale_mat * 0.1;
        scale_mat[15] = 1.0;

        self.cube_shader.set_used();
        let proj = self.camera().projection();
        let view = self.camera().view();
        self.cube_shader.set_projection_and_view(gl, proj, view);

        let trans_mat_world = na::Matrix4::new_translation(&pos);


        cube_model.render(gl, &self.cube_shader, trans_mat_world * scale_mat);

    }

    fn render_hitboxes(&mut self, render_context: &mut render_gl::context::Context) {
        let mut switched = false;
        if !render_context.wire_frame {
            switched = true;
            render_context.switch_mode();
        }


        //RENDER HITBOXES

        let gl = &render_context.gl;

        self.hitbox_shader.set_used();
        self.hitbox_shader.set_projection_and_view(gl, self.camera().projection(), self.camera().view());



        for entity in self.entities.hitbox_entities() {
            match entity.is_hit {
                true => self.hitbox_shader.set_vec3(gl, "color", na::Vector3::new(1.0, 0.0, 0.0)),
                false => self.hitbox_shader.set_vec3(gl, "color", na::Vector3::new(1.0, 1.0, 1.0))
            };

            for hitbox in &entity.hitboxes {
                let col_box = hitbox.make_transformed(entity.physics.pos, entity.physics.rotation);

                let clr = na::Vector3::new(1.0, 1.0, 0.0);
                let cube_model = cube::Cube::from_collision_box(col_box, clr, gl);

                cube_model.render(gl, &self.hitbox_shader, na::Matrix4::identity());
            }
        }

        if switched {
            render_context.switch_mode();
        }

    }

    pub fn reload_shaders(&mut self, render_context: &render_gl::context::Context) {

        let shaders = vec![ ("mesh_shader", &mut self.mesh_shader), ("hitbox_shader", &mut self.hitbox_shader)];

        for (name, shader) in shaders {
            match render_gl::Shader::new(name, &render_context.res, &render_context.gl) {
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




fn empty(render_context: &render_gl::context::Context) -> Result<Scene, failure::Error> {

    let width = 700;
    let height = 800;
    let entities = entity::Entities::new();

    let follow_camera = camera::FollowCamera::new(width, height); //
    let free_camera = camera::FreeCamera::new();


    let cube_shader = render_gl::Shader::new("light_color_shader", &render_context.res, &render_context.gl)?;

    let mesh_shader = render_gl::Shader::new("mesh_shader", &render_context.res, &render_context.gl)?;

    let hitbox_shader = render_gl::Shader::new("hitbox_shader", &render_context.res, &render_context.gl)?;

    let actions = action_system::load_player_actions(&render_context.res)?;

    let cameras = Cameras {
        free_camera,
        follow_camera,
        mode: camera::CameraMode::Follow
    };


    Ok(Scene {
        mesh_shader,
        actions,
        cube_shader,
        hitbox_shader,
        entities,
        cameras,
        models: std::collections::HashMap::new(),
        animations: std::collections::HashMap::new(),
        ais: std::collections::HashMap::new(),
        render_hitboxes: false,
        world_triangles: Vec::new(),
    })
}

fn load_animations(file_path: &str, skeleton: &render_gl::Skeleton, base_animations: Option<&render_gl::PlayerAnimations>) -> Option<render_gl::PlayerAnimations>{

    let animations = match render_gl::load_animations(file_path, &skeleton, base_animations) {
        Ok(key_frames) => key_frames,
        Err(err) => {           //
            println!("Error loading key_frames: {:#?}", err);
            return None; }
    };

    Some(animations)
}
