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

    pub fn millis(&self) -> i32 {
        self.value.as_millis() as i32
    }

    pub fn update(&mut self) {
        self.value = self.last_update.elapsed();
        self.last_update = Instant::now();
    }

}
