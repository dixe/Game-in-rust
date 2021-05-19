use crate::game;
use crate::entity::{Entity};
use crate::game::ai::behaviour::{Behaviour, IdleBehaviour, WalkToBehaviour};
use nalgebra_glm as glm;

#[derive(Clone)]
pub enum Ai {
    Idle(IdleBehaviour),
    WalkTo(WalkToBehaviour)
}

impl Ai {

    pub fn idle() -> Ai {
        Ai::Idle(IdleBehaviour {})
    }
}



pub fn run_ais(scene: &mut game::Scene) {

    let player = &scene.entities.player;

    for enemy in scene.entities.enemies.values_mut() {

        let ai: &Ai = match scene.ais.get(&enemy.id) {
            Some(a) => a,
            _ => {
                continue;
            }
        };

        let mut next = None;
        match ai {
            Ai::Idle(b) =>
            {
                b.execute(enemy);
                next = Some(Ai::WalkTo(WalkToBehaviour { location: na::Vector3::new(15.0, 0.0, 0.0)}));

            }
            Ai::WalkTo(b) =>
            {
                b.execute(enemy);
                if b.finished(enemy) {
                    let next_x;
                    if enemy.physics.pos.x > 10.0 {
                        next_x = 0.0;
                    }
                    else {
                        next_x = 15.0;
                    }

                    next = Some(Ai::WalkTo(WalkToBehaviour { location: na::Vector3::new(next_x, 0.0, 0.0)}));
                }

            }
        }

        match next {
            Some(b) => {
                scene.ais.insert(enemy.id, b);
            },
            _ => {}
        };
    }
}
