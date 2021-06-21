use crate::game;
use crate::entity::{Entity};
use crate::game::ai::behaviour::{IdleBehaviour, WalkToBehaviour};
use nalgebra_glm as glm;
use crate::shared::Behaviour;


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
                b.execute(&mut enemy.base_entity);
                next = Some(Ai::WalkTo(WalkToBehaviour { location: na::Vector3::new(15.0, 0.0, 0.0)}));

            }
            Ai::WalkTo(b) =>
            {
                b.execute(&mut enemy.base_entity);
                if b.finished(&mut enemy.base_entity) {
                    let next_x;
                    if enemy.base_entity.physics.pos.x > 10.0 {
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
