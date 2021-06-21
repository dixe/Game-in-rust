use std::time::{
    Instant,
    Duration
};

pub struct Deltatime {
    value: Duration,
    last_update: Instant,
    time_speed: f32,

}

impl Deltatime {
    pub fn new() -> Self {
        Self {
            value: Duration::new(0, 0),
            last_update: Instant::now(),
            time_speed: 1.0
        }
    }


    pub fn time(&self) -> f32 {
        (self.value.as_millis() as f32 )/ 1000.0 * self.time_speed
    }

    pub fn update(&mut self) {
        self.value = self.last_update.elapsed();
        self.last_update = Instant::now();
    }

}
