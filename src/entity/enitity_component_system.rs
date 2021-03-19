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

    #[component = "EntityType"]
    pub entity_type: std::collections::HashMap<usize, EntityType>,

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
            entity_type: std::collections::HashMap::new(),
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


    pub fn render(&self, entity_id: usize, gl: &gl::Gl, shader: &render_gl::Shader) {

        // TODO see if this can be made wihtout default
        let default = EntityType::Simple(entity_id);
        let e_type = match self.get_entity_type(entity_id) {
            Some(e_type) => e_type,
            None => &default
        };

        match e_type {
            EntityType::Simple(id) => self.render_simple(*id, None, gl, shader),
            EntityType::Complex(complex) => self.render_complex(complex, None, gl, shader)
        }

    }


    fn render_complex(&self, complex: &ComplexEntity, anchor_physics: Option<&Physics>, gl: &gl::Gl, shader: &render_gl::Shader) {

        self.render_simple(complex.id, None, gl, shader);

        let base_physics = match self.get_physics(complex.id) {
            Some(physics) => physics,
            _ => return,
        };

        for extra_id in &complex.sub_entities {


            // maybe self.render and also maybe self.render should take a base option anchor_physics
            // to make the signatures m
            self.render_simple(*extra_id, Some(&base_physics), gl, shader);
        }

    }

    fn render_simple(&self, entity_id: usize, anchor_physics: Option<&Physics>, gl: &gl::Gl, shader: &render_gl::Shader) {

        let physics = match self.get_physics(entity_id) {
            Some(physics) => physics,
            _ => return,
        };


        match (self.models.get(physics.model_id), self.get_animation(physics.entity_id)) {
            (Some(m), Some(ani)) => {
                let model_mat = ani.calculate_model_mat(physics, anchor_physics);
                m.render_from_model_mat(gl, shader, model_mat);
            },
            (Some(m),None) => {
                let model_mat = render_gl::calculate_model_mat(physics, anchor_physics);
                m.render_from_model_mat(gl, shader, model_mat);},
            _ => {}
        };
    }

}
