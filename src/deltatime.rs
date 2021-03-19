use std::time::{
    Instant,
    Duration
};

pub struct Deltatime {
    value: Duration,
    last_update: Instant
}

impl Deltatime {
    pub fn new() -> Self {
        Self {
            value: Duration::new(0, 0),
            last_update: Instant::now()
        }
    }


    pub fn time(&self) -> f32 {
        (self.value.as_millis() as f32 )/ 1000.0
    }

    pub fn update(&mut self) {
        self.value = self.last_update.elapsed();
        self.last_update = Instant::now();
    }

}
