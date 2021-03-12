
#[derive(Debug,Copy, Clone)]
pub struct Shooter {

    pub cool_down: f32,
    pub velocity: f32
}


impl Shooter {

    pub fn default() -> Shooter {

        Shooter {
            cool_down: 1.0,
            velocity: 30.0
        }
    }
}
