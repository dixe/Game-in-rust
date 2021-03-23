use crate::entity;
use crate::render_gl;



#[derive(PartialEq)]
pub enum PlayerState {
    Moving,
    Attacking
}



pub struct State {
    pub player_shots: std::collections::HashSet::<usize>,
    pub enemy_shots: std::collections::HashSet::<usize>,
    pub enemies: std::collections::HashSet<usize>,
    pub player_state: PlayerState,


}


impl State {

    pub fn new() -> State {
        State {
            enemies: std::collections::HashSet::new(),
            player_shots: std::collections::HashSet::new(),
            enemy_shots: std::collections::HashSet::new(),
            player_state: PlayerState::Moving
        }
    }


    pub fn remove_shot(&mut self, shot_id: &usize) {
        self.player_shots.remove(shot_id);
        self.enemy_shots.remove(shot_id);
    }




    pub fn render(&self, ecs: &entity::EntityComponentSystem,  gl: &gl::Gl, shader: &render_gl::Shader) {


        for id in &self.enemies {
            render_gl::render(ecs, *id, gl, shader)
        }

        for id in &self.player_shots {
            render_gl::render(ecs, *id, gl,shader);
        }

        for id in &self.enemy_shots {
            render_gl::render(ecs, *id, gl,shader);
        }
    }
}
