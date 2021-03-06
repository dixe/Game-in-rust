use nalgebra as na;


use crate::render_gl::Renderable;


use crate::entity;

pub struct Shot {
    pub entity: entity::Entity,
    pub time_remaining: i32,
    pub expired: bool,
}


impl Shot {

    pub fn new(entity: entity::Entity, life_time: i32) -> Self {
        Shot {
            entity,
            time_remaining: life_time,
            expired: false
        }
    }


    pub fn update(&mut self, delta: i32) {

        self.time_remaining -= delta;
        self.expired = self.time_remaining <= 0;
    }
}



impl Renderable for Shot {
    fn render(&self, gl: &gl::Gl, projection: na::Matrix4<f32>, view: na::Matrix4<f32>) {
        if ! self.expired {
            self.entity.render(gl, projection, view);
        }
    }

}
