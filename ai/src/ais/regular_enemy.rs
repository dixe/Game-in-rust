use shared::*;
use shared::physics_functions;
use nalgebra as na;

pub struct RegularEnemyAi {

}

impl RegularEnemyAi {

    pub fn new() -> Self {
        RegularEnemyAi{}
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

                //TODO take all of this into a function to be shared between ais

                let player_dist = (run_data.entity.physics.pos - run_data.player.physics.pos).magnitude();

                let mut vel = run_data.entity.physics.pos - run_data.player.physics.pos;
                println!("I am going to get you in {:?}", vel.magnitude());


                let leway = 2.0;

                if player_dist > (ai_data.distance - leway) && player_dist < (ai_data.distance + leway) {
                    physics_functions::set_velocity(&mut run_data.entity.physics, na::Vector3::new(0.0, 0.0,0.0));
                    println!("RETURN");
                    return;
                }


                if player_dist < (ai_data.distance - leway) {
                    physics_functions::set_velocity(&mut run_data.entity.physics, vel );

                }

                if player_dist > (ai_data.distance + leway) {

                    physics_functions::set_velocity(&mut run_data.entity.physics, -vel);
                }
            },
            shared::Behaviour::Attack => {
            },
        }


    }
}
