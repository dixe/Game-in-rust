use core::fmt::Debug;

use crate::game;
use crate::entity;
use crate::action_system;


pub fn update_actions(actions: &mut std::collections::HashMap<usize, entity::ActionsInfo>, physics: &mut std::collections::HashMap<usize, entity::Physics>, state: &mut game::State, delta: f32, impls: &ActionsImpl) {
    for action in actions.values_mut() {
        match physics.get_mut(&action.entity_id) {
            Some(physics) => {
                action.update(physics, state, delta, impls);
            },
            _ => {}
        };
    }
}




#[derive(Debug, Copy, Clone)]
pub enum Curve {
    Linear(na::Vector3<f32>, na::Vector3<f32>),
    Cubic(na::Vector3<f32>, na::Vector3<f32>, na::Vector3<f32>),
}

#[derive(Debug, Copy, Clone)]
pub struct Part {
    pub curve: Curve,
    pub start: f32,
    pub end: f32
}


#[derive(Debug, Clone)]
pub struct BezierAction {
    pub parts: Vec<Part>,
}

impl BezierAction {
    pub fn update(&self, t: f32, physics: &mut entity::Physics, init: &entity::Physics) {
        // TODO each part should be done in the specified amount of time
        // println!("{}", t);
        for p in self.parts.iter() {

            if p.start <= t && t <= p.end {

                let bz = match p.curve {
                    Curve::Linear(p0, p1) => action_system::bezier_linear(t, p0, p1),
                    Curve::Cubic(p0, p1, p2) => action_system::bezier_cubic(t, p0, p1, p2),
                };
                // println!("{:#?}", physics.pos);
                physics.pos = init.pos + bz;
            }
        }
    }
}


impl Action for BezierAction {
    fn update(&self, time_passed: f32, physics: &mut entity::Physics, init: &entity::Physics) {
        self.update(time_passed, physics, init);
    }
}


#[derive(Clone)]
pub struct FuncAction {
    pub update_fn: fn(time_passed: f32, physics: &mut entity::Physics, init: &entity::Physics)
}


impl Debug for FuncAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Func action")
    }
}


impl Action for FuncAction {
    fn update(&self, time_passed: f32, physics: &mut entity::Physics, init: &entity::Physics) {
        (self.update_fn)(time_passed, physics, init);
    }
}

pub trait Action {
    fn update(&self, time_passed: f32, physics: &mut entity::Physics, init: &entity::Physics);
}


#[derive(Debug, Copy, Clone)]
pub enum Actions {
    Swing,
    Idle,
}

#[derive(Debug, Clone)]
pub struct ActionsImpl {
    pub swing: BezierAction,
    pub idle: FuncAction,
}


impl ActionsImpl {

    pub fn update(&self, action: Actions, t: f32, physics: &mut entity::Physics, init: &entity::Physics) {
        match action {
            Actions::Swing => self.swing.update(t, physics, init),
            Actions::Idle => {
                self.idle.update(t, physics, init);},
        }

    }
}
