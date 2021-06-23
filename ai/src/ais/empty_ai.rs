use shared::*;


pub struct EmptyAi {

}


impl EmptyAi {

    pub fn new() -> Self {
        EmptyAi {

        }
    }
}

impl Ai for EmptyAi {

    fn run(&self, entity: &mut BaseEntity) {
        println!("Rewrite");
    }
}
