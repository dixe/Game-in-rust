
#[derive(Debug,Copy, Clone)]
pub struct Shooter {

    cool_down_remaining: f32,
    cool_down_time: f32,
    pub speed: f32,
    pub distance: f32
}


impl Shooter {

    pub fn default_player() -> Shooter {

        Shooter {
            cool_down_time: 0.500,  // time in sec
            cool_down_remaining: 0.0,
            speed: 20.0,
            distance: 20.0
        }
    }

    pub fn default_enemy() -> Shooter {
        Shooter {
            cool_down_time: 0.800,  // time in sec
            cool_down_remaining: 0.0,
            speed: 8.0,
            distance: 400.0
        }
    }
    pub fn shoot(&mut self) {
        self.cool_down_remaining = self.cool_down_time;
    }

    pub fn update(&mut self, delta: f32) {

        if self.cool_down_remaining > 0.0 {
            self.cool_down_remaining -= delta;
        };
    }


    pub fn can_shoot(&self) -> bool {
        return self.cool_down_remaining <= 0.0
    }

}
