
#[derive(Debug,Copy, Clone)]
pub struct Shooter {

    cool_down_remaining: i32,
    cool_down_time: i32,
    pub speed: f32,
    pub distance: f32
}


impl Shooter {

    pub fn default() -> Shooter {

        Shooter {
            cool_down_time: 700,  // time in ms
            cool_down_remaining: 0,
            speed: 20.0,
            distance: 20.0
        }
    }
    pub fn shoot(&mut self) {
        self.cool_down_remaining = self.cool_down_time;
    }

    pub fn update(&mut self, delta: i32) {

        if self.cool_down_remaining > 0 {
            self.cool_down_remaining -= delta;
        };
    }


    pub fn can_shoot(&self) -> bool {
        return self.cool_down_remaining <= 0
    }

}
