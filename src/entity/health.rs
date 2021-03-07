


#[derive(Copy, Clone)]
pub struct Health {
    health: f32
}


impl Health {
    pub fn new(max_health: f32) -> Self {
        Health {

            health: max_health
        }
    }


    pub fn damage(&mut self, dmg: f32) -> bool {

        self.health -= dmg;


        let dead = self.health <= 0.0;
        println!("{}, {}", dead, self.health);

        dead
    }


}
