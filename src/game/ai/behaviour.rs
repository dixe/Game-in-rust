use nalgebra as na;
use crate::entity::{Entity, EntityState};
use crate::game::ai::behaviour_functions as bf;


pub trait Behaviour {

    fn execute(&self, entity: &mut Entity);

    fn finished(&self, entity: &Entity) -> bool;
}


#[derive(Clone)]
pub struct WalkToBehaviour {
    pub location: na::Vector3::<f32>
}



impl Behaviour for WalkToBehaviour {


    fn execute(&self, entity: &mut Entity) {
        match entity.get_state() {
            EntityState::Moving => {},
            _ => {
                entity.queued_action = Some(EntityState::Moving);
            },
        }


        bf::move_to_point(entity, self.location);
    }

    fn finished(&self, entity: &Entity) -> bool {
        let mag = (entity.physics.pos.xy() - self.location.xy()).magnitude();
        mag < 0.01
    }



}



#[derive(Clone)]
pub struct IdleBehaviour {
}

impl IdleBehaviour {
    pub fn new() -> IdleBehaviour {
        IdleBehaviour {

        }
    }
}

impl Behaviour for IdleBehaviour {

    fn execute(&self, entity: &mut Entity) {

    }

    fn finished(&self, entity: &Entity) -> bool {
        true
    }
}
