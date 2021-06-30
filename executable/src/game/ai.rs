use crate::game;


use crate::shared::{Ai};
use crate::resources::Resources;



pub struct LoadedAis {
    pub regular_enemy: AiPlugin<shared::RegularEnemyState>,
}

pub struct AiPlugin<T> {
    pub ai: Box<dyn shared::Ai<T>>,
    pub lib: libloading::Library,
}


impl<T> AiPlugin<T> {

    fn run(&self,run_data: shared::AiRunData, ai_data: &mut T) {
        self.ai.run(run_data, ai_data);
    }
}


pub fn load_ais(res: &Resources) -> LoadedAis {
    LoadedAis {
        regular_enemy: load_regular_enemy_ai(res)
    }
}


fn load_regular_enemy_ai(res: &Resources) -> AiPlugin<shared::RegularEnemyState> {
    // make a copy of dll so we can still build it

    let lib = res.copy_and_load_lib("ai.dll");

    let regular_enemy_ai: libloading::Symbol<extern "Rust" fn() ->  Box<dyn shared::Ai<shared::RegularEnemyState>>> = unsafe { lib.get(b"regular_enemy_ai") }
    .expect("load symbol");

    AiPlugin {
        ai: regular_enemy_ai(),
        lib: lib
    }

}


pub fn run_ais(scene: &mut game::Scene) {

    let ais = match &scene.loaded_ais {
        Some(loaded) => loaded,
        None => {
            return;
        }
    };

    for enemy in scene.entities.enemies.values_mut() {

        let run_info = match &mut enemy.ai {
            Some(a) => match a {
                shared::EntityAi::RegularEnemy(state) => {

                    AiRunInfo {
                        ai: &ais.regular_enemy,
                        ai_data: state
                    }
                },
                shared::EntityAi::BossEnemy => {
                    continue;
                }
            }
            _ => {
                continue;
            }
        };

        let run_data = shared::AiRunData {
            entity: &mut enemy.base_entity,
            player: &scene.entities.player.base_entity
        };

        run_info.ai.run(run_data, run_info.ai_data);
    }
}

struct AiRunInfo<'a, T> {
    ai: &'a AiPlugin<T>,
    ai_data: &'a mut T,
}
