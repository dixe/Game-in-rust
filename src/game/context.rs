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

    pub enemies_ids: Vec<usize>,
    pub player_id: usize,

    // STUFF WE NEED
    pub controls: controls::Controls,
    pub scene: scene::Scene,
    pub level: level::Level,
    pub render_context: render_gl::context::Context,
    pub camera: camera::Camera,

    pub entity_manager: entity::EntityManager,


    player_projectile_model_id: usize,

    delta_time: deltatime::Deltatime,

}

impl Context {


    fn setup_player(&mut self) -> Result<(), failure::Error>  {


        let player_pos = na::Vector3::new(3.0, 3.0, 0.0);
        let player_color = na::Vector3::new(0.0,  1.0, 1.0);

        let player_cube = cube::Cube::new(&self.render_context.res, player_color, &self.render_context.gl)?;


        let player_model_id = self.entity_manager.add_model(player_cube);



        let player_id = self.entity_manager.add_entity(player_model_id, player_pos);

        self.player_id = player_id;

        Ok(())
    }


    pub fn new() -> Result<Context, failure::Error> {

        let mut ctx = empty()?;

        ctx.setup_player();

        Ok(ctx)
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


    pub fn update_game_state(&mut self) {

        let delta = self.get_delta_millis();
        for p in &mut self.player_projectiles {
            p.update(delta);

            if(p.expired) {
                self.entity_manager.remove_entity(p.entity_id);
            }
        }

        self.player_projectiles.retain(|p| !p.expired);


        match self.controls.shoot_dir {
            Some(dir) =>
            {
                //todo check cooldown/shooting speed

                // spawn projectile with dir



                let clr = na::Vector3::new(0.0,  0.0, 0.0);
                let cube = cube::Cube::new(&self.render_context.res, clr, &self.render_context.gl).unwrap();

                let player_pos = match self.entity_manager.get_entity(self.player_id) {
                    Some(p) => p.pos,
                    _ => return // Can we shoot when dead, and should all exit. Maybe just update shooting in own function
                };


                let p_id = self.entity_manager.add_entity_with_vel(self.player_projectile_model_id, player_pos, dir);

                let shot = shot::Shot::new(p_id, 300);

                self.player_projectiles.push(shot);




                /*
                let mut projectile_e = entity::Entity::new(cube, self.player.pos);


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

                entity_manager.add_player_projectile(cube, self.player.pos, d
                let mut projectile_e = entity::Entity::new(cube, self.player.pos);
                projectile_e.velocity = dir;
                let projectile = shot::Shot::new(projectile_e, 300);

                self.player_projectiles.push(projectile);
            }


                 */
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


        // player
        self.entity_manager.render(self.player_id, &self.render_context.gl, &self.camera.projection(), &self.camera.view());


        // enemies
        for id in &self.enemies_ids {
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

    let mut entity_manager = entity::EntityManager::new();

    let enemy_pos = na::Vector3::new(0.0, 0.0, 0.0);

    let enemy_color = na::Vector3::new(0.0, 0.0, 0.0);

    background_color_buffer.set_used(&render_context.gl);

    let camera = camera::Camera::new();

    let event_pump = render_context.sdl.event_pump().unwrap();

    let level = level::Level::load(&render_context.res,"levels/debugLevel1.txt")?;

    let scene = scene::Scene::new(&level, &render_context)?;

    let controls = controls::Controls::new(event_pump);

    let enemy_cube = cube::Cube::new(&render_context.res, enemy_color, &render_context.gl)?;



    let delta_time = deltatime::Deltatime::new();

    let mut enemies_ids = Vec::new();

    let enemy_model_id = entity_manager.add_model(enemy_cube);

    enemies_ids.push(entity_manager.add_entity(enemy_model_id, enemy_pos));

    Ok(Context {
        player_projectiles: Vec::<shot::Shot>::new(),
        player_id: 0,
        scene,
        controls,
        render_context,
        level,
        camera,
        delta_time,
        entity_manager,
        enemies_ids,
        player_projectile_model_id: 0,
    })


}
