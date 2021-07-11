use shared::*;
use rand::Rng;
use crate::behaviours;

pub struct RegularEnemyAi {

}

impl RegularEnemyAi {

    pub fn new() -> Self {
        RegularEnemyAi { }
    }
}



impl Ai<RegularEnemyState> for RegularEnemyAi {

    fn run(&self, run_data: AiRunData, ai_data: &mut RegularEnemyState) {

        match ai_data.current_behaviour {
            shared::Behaviour::Empty => {
                ai_data.current_behaviour = shared::Behaviour::KeepDistance;
            },
            shared::Behaviour::Patrol => {
            },
            shared::Behaviour::KeepDistance => {


                let keep_dist_res = behaviours::keep_distance(ai_data.distance, run_data.entity, run_data.player);

                // maybe engage in an attack
                if keep_dist_res == behaviours::KeepDistanceResult::InDistance {
                    let mut rng = rand::thread_rng();

                    let random = rng.gen::<f32>();


                    if random > 0.99 {
                        ai_data.current_behaviour = shared::Behaviour::Attack;
                    }

                }

            },
            shared::Behaviour::Attack => {

                let attack_res =  behaviours::attack(run_data.entity, run_data.player);

                match attack_res {
                    behaviours::AttackResult::ClosingDistance => {},
                    behaviours::AttackResult::Attacking => {
                        ai_data.current_behaviour = shared::Behaviour::KeepDistance;
                    },

                }



            },
        }
    }
}
