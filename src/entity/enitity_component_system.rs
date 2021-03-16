use crate::render_gl;
use crate::entity::*;


#[derive(ComponentSystem)]
pub struct EntityComponentSystem {

    next_id: usize,
    models: Vec<Model>,

    // Components
    #[component = "Physics"]
    pub physics: std::collections::HashMap<usize, Physics>,
    #[component = "Health"]
    health: std::collections::HashMap<usize, Health>,
    #[component = "Shooter"]
    pub shooter: std::collections::HashMap<usize, Shooter>,
    #[component = "Shot"]
    pub shot: std::collections::HashMap<usize, Shot>,
    #[component = "Animation"]
    pub animation: std::collections::HashMap<usize, Animation>,

    model_reference: std::collections::HashMap<usize, usize>,


}


impl EntityComponentSystem {

    pub fn new () -> Self {
        return EntityComponentSystem {
            next_id: 1,
            physics: std::collections::HashMap::new(),
            health: std::collections::HashMap::new(),
            shooter: std::collections::HashMap::new(),
            shot: std::collections::HashMap::new(),
            animation: std::collections::HashMap::new(),
            models: Vec::<Model>::new(),
            model_reference: std::collections::HashMap::new(),
        }
    }

    pub fn add_entity (&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn set_model(&mut self, entity_id: usize, model_id: usize)  {
        self.model_reference.insert(entity_id, model_id);
    }


    pub fn add_model(&mut self, model: Model) -> usize {
        self.models.push(model);

        (self.models.len() - 1) as usize

    }


    pub fn render(&self, entity_id:usize, gl: &gl::Gl, shader: &render_gl::Shader) {
        match self.get_physics(entity_id) {
            Some(e) =>
                match (self.models.get(e.model_id), self.get_animation(e.entity_id)) {
                    (Some(m), Some(ani)) => {

                        let model_mat = ani.calculate_model_mat(e.pos, e.rotation_sin, e.rotation_cos, e.scale);
                        m.render_from_model_mat(gl, shader, model_mat);
                    },
                    (Some(m),None) => m.render(gl, shader, e.pos, e.rotation_sin, e.rotation_cos, e.scale),
                    _ => {}
                },
            None => {}
        };

    }

}
