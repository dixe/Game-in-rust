use core::fmt::Debug;

use crate::entity;



#[derive(Debug, Clone)]
pub struct ActionsInfo {
    pub entity_id: usize,
    pub default: Option<ActionData>,
    pub queue: std::collections::VecDeque<ActionData>,
    pub active: Option<ActionData>
}


impl ActionsInfo {

    pub fn new(entity_id: usize, default: Option<ActionData>) -> ActionsInfo {
        ActionsInfo {
            entity_id,
            default,
            queue: std::collections::VecDeque::new(),
            active: default

        }
    }

    fn next(&mut self) {
        match self.active {
            Some(data) => {
                if data.time_passed < data.total_time {
                    return
                }
            },
            _ => {}
        };

        // if we are here active was none, or it is expired
        // set next in queue, or default

        self.active = match self.queue.pop_front() {
            None => self.default,
            data => data
        };
    }

    pub fn update(&mut self, physics: &mut entity::Physics, delta: f32) {

        match self.active {
            Some(mut data) => {
                data.update(physics, delta);
                self.active = Some(data);
            },
            _ => {},
        }

        self.next();
    }


}

#[derive(Copy, Clone)]
pub struct ActionData {
    pub time_passed: f32,
    pub total_time: f32,
    init: entity::Physics,
    update_fn: fn(time_passed: f32, physics: &mut entity::Physics, init: &entity::Physics),
}


impl Debug for ActionData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "time_passed: {}\ntotal_time: {}", self.time_passed, self.total_time)
    }
}

impl ActionData {

    pub fn new(update_fn: fn(time_passed: f32, physics: &mut entity::Physics,init: &entity::Physics), init: entity::Physics) -> ActionData {

        ActionData {
            time_passed: 0.0,
            total_time: 1.0,
            init,
            update_fn
        }
    }

    pub fn update(&mut self, physics: &mut entity::Physics, delta: f32) {
        self.time_passed += delta;

        let t = self.time_passed.clamp(0.0, self.total_time);

        (self.update_fn)(t, physics, &self.init);

    }
}




/*
impl Action for ActionData {

fn update(&mut self, mut physics: entity::Physics, delta: i32) {

self.time_passed += delta;

physics.pos.z += 0.5 * (self.time_passed as f32 / 300.0).sin();
}

    fn entity_id(&self) -> usize {
    self.entity_id
}

}

    pub trait Action {

    fn update(&mut self, physics: entity::Physics, delta: i32);

    fn entity_id(&self) -> usize;

    fn calculate_model_mat(&self, physics: entity::Physics, anchor_physics: Option<&entity::Physics>) -> na::Matrix4::<f32> {
    render_gl::calculate_model_mat(&physics, anchor_physics)
}
}
     */
