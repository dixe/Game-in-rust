#[derive(Debug,Clone,Copy)]
pub struct Shot {
    pub entity_id: usize,
    pub time_remaining: f32,
    pub expired: bool,
    pub used: bool,
    pub damage: f32
}


impl Shot {

    pub fn new(entity_id: usize, life_time: f32) -> Self {
        Shot {
            entity_id,
            time_remaining: life_time,
            expired: false,
            used: false,
            damage: 10.0
        }
    }


    pub fn update(&mut self, delta: f32) {

        self.time_remaining -= delta;
        self.expired = self.time_remaining <= 0.0;
    }
}
