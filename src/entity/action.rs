use core::fmt::Debug;

use crate::entity;
use crate::game;
use crate::action_system;

#[derive(Debug, Clone)]
pub struct ActionsInfo {
    pub entity_id: usize,
    pub default: Option<ActionData>,
    pub queue: std::collections::VecDeque<ActionData>,
    pub active: Option<ActionData>,
}


impl ActionsInfo {

    pub fn new(entity_id: usize, default: Option<ActionData>) -> ActionsInfo {
        ActionsInfo {
            entity_id,
            default,
            queue: std::collections::VecDeque::new(),
            active: default,
        }
    }

    fn next(&mut self, physics: &mut entity::Physics,) {
        match self.active {
            Some(data) => {
                if data.time_passed <= data.total_time {
                    return
                }
                else {
                    // reset to init physics
                    data.reset(physics);
                    data.expired();
                }
            },
            None => {}
        };

        // set next action

        self.active = match self.queue.pop_front() {
            None => self.default,
            data => data
        };
    }

    pub fn update(&mut self, physics: &mut entity::Physics, delta: f32, impls: &action_system::ActionsImpl) {

        match self.active {
            Some(mut data) => {
                data.update(physics, delta, impls);
                self.active = Some(data)
            },
            _ => {},
        }

        self.next(physics);
    }

}

#[derive(Copy, Clone)]
pub struct ActionData {
    pub time_passed: f32,
    pub total_time: f32,
    init: entity::Physics,
    action: action_system::Actions, //usizefn(time_passed: f32, physics: &mut entity::Physics, init: &entity::Physics),
    done_fn: Option<fn()>,
}






impl Debug for ActionData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "time_passed: {}\ntotal_time: {}", self.time_passed, self.total_time)
    }
}

impl ActionData {

    pub fn new(action: action_system::Actions, done_fn: Option<fn()>, init: entity::Physics) -> ActionData {

        ActionData {
            time_passed: 0.0,
            total_time: 1.0,
            init,
            action,
            done_fn
        }
    }


    pub fn percent_done(&self) -> f32 {
        self.time_passed / self.total_time
    }


    pub fn reset(&self, physics: &mut entity::Physics) {
        physics.pos = self.init.pos;
        physics.rotation = self.init.rotation;
        physics.scale = self.init.scale;
    }


    pub fn update(&mut self, physics: &mut entity::Physics, delta: f32, impls: &action_system::ActionsImpl) {
        self.time_passed += delta;

        let t = self.time_passed / self.total_time;

        impls.update(self.action, t, physics, &self.init);

    }


    pub fn expired(&self) {
        match self.done_fn {
            Some(func) => {
                (func)();
            },
            _ => {}
        };

    }
}
