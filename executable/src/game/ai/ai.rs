use crate::game;

use nalgebra_glm as glm;
use crate::shared::{Ai};
use crate::resources::Resources;



pub struct LoadedAis {
    pub empty: AiPlugin,
}

#[derive(Clone, Copy)]
pub enum EntityAi {
    Empty
}

pub struct AiPlugin {
    pub ai: Box<dyn shared::Ai>,
    pub lib: libloading::Library,
}


impl shared::Ai for AiPlugin {

    fn run(&self, entity: &mut shared::BaseEntity) {
        self.ai.run(entity);
    }

}


pub fn load_ais(res: &Resources) -> LoadedAis {
    LoadedAis {
        empty: load_empty_ai(res)
    }
}


fn load_empty_ai(res: &Resources) -> AiPlugin {
    // make a copy of dll so we can still build it

    let lib = res.copy_and_load_lib("ai.dll");

    let empty_ai: libloading::Symbol<extern "Rust" fn() ->  Box<dyn shared::Ai>> = unsafe { lib.get(b"empty_ai") }
    .expect("load symbol");

    AiPlugin {
        ai: empty_ai(),
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



        let ai: &Ai = match &enemy.ai {
            Some(a) => match a {
                EntityAi::Empty => &ais.empty,

            }
            _ => {
                continue;
            }
        };


        ai.run(&mut enemy.base_entity);

        /*
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
         */
    }
}
