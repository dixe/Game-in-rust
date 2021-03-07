use nalgebra as na;


use crate::entity;

pub struct Shot {
    pub entity_id: usize,
    pub time_remaining: i32,
    pub expired: bool,
}


impl Shot {

    pub fn new(entity_id: usize, life_time: i32) -> Self {
        Shot {
            entity_id,
            time_remaining: life_time,
            expired: false
        }
    }


    pub fn update(&mut self, delta: i32) {

        self.time_remaining -= delta;
        self.expired = self.time_remaining <= 0;
    }
}
