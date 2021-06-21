use nalgebra as na;
use crate::game::ai::behaviour_functions as bf;
use crate::shared;


#[derive(Clone)]
pub struct WalkToBehaviour {
    pub location: na::Vector3::<f32>
}



impl shared::Behaviour for WalkToBehaviour {

    fn execute(&self, entity: &mut shared::BaseEntity) {
        match entity.state {
            shared::EntityState::Moving => {},
            _ => {
                entity.queued_action = Some(shared::EntityState::Moving);
            },
        }


        bf::move_to_point(entity, self.location);
    }

    fn finished(&self, entity: &shared::BaseEntity) -> bool {
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

impl shared::Behaviour for IdleBehaviour {

    fn execute(&self, entity: &mut shared::BaseEntity) {

    }

    fn finished(&self, entity: &shared::BaseEntity) -> bool {
        true
    }
}
