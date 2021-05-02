use core::fmt::Debug;


use crate::entity;
use crate::action_system;


pub fn update_actions(actions: &mut std::collections::HashMap<usize, entity::ActionsInfo>, physics: &mut std::collections::HashMap<usize, entity::Physics>, delta: f32, impls: &ActionsImpl) {
    for action in actions.values_mut() {
        match physics.get_mut(&action.entity_id) {
            Some(physics) => {
                action.update(physics, delta, impls);
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
    pub positions: Curve,
    pub normals: Curve,
    pub start: f32,
    pub end: f32
}


#[derive(Debug, Clone)]
pub struct BezierAction {
    pub parts: Vec<Part>,
}


impl BezierAction {
    pub fn update(&self, time_passed: f32, physics: &mut entity::Physics, init: &entity::Physics) {

        let base = na::Vector3::new(0.0, 0.0, 1.0);

        for p in self.parts.iter() {
            if p.start <= time_passed && time_passed <= p.end {

                let t = clamp01(time_passed, p.start, p.end);

                let bz = match p.positions {
                    Curve::Linear(p0, p1) => action_system::bezier_linear(t, p0, p1),
                    Curve::Cubic(p0, p1, p2) => action_system::bezier_cubic(t, p0, p1, p2),
                };


                let bz_normal = match p.normals {
                    Curve::Linear(p0, p1) => action_system::bezier_linear(t, p0, p1),
                    Curve::Cubic(p0, p1, p2) => action_system::bezier_cubic(t, p0, p1, p2),
                };


                //TODO handle NAN when bz_normal is also 0 0 1


                let rot_axis = base.cross(&bz_normal);

                let angle = (bz_normal.dot(&base)).acos();

                let rot = na::UnitQuaternion::from_axis_angle(&na::Unit::new_normalize(rot_axis), angle);

                physics.rotation = rot;

                physics.pos = init.pos + bz;
            }
        }
    }
}

fn clamp01(t: f32, min: f32, max: f32) -> f32{
    f32::max(f32::min(1.0, (t - min) / (max - min)), 0.0)
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






pub fn from_anchor_points(in_an: &Vec<entity::AnchorPoint>, base_anchor: entity::AnchorPoint) -> BezierAction {


    let mut parts = Vec::new();

    // for every pair create a linear bezier part

    let mut anchors = in_an.clone();

    anchors.push(base_anchor);

    let len = (anchors.len() - 1) as f32;
    let step = 1.0 / len;


    let mut i = 1;

    while i < anchors.len() {



        let prev = anchors[i - 1];
        let this = anchors[i];

        let norm_diff = this.normal - prev.normal;

        let _rotation = na::Rotation3::new(norm_diff);


        let positions = Curve::Linear(prev.pos - base_anchor.pos, this.pos - base_anchor.pos);
        let normals = Curve::Linear(prev.normal, this.normal);


        let start = ((i-1) as f32) * step;
        let end = (i as f32) * step;

        let part =  Part {
	    positions,
            normals,
            start,
            end
        };

        parts.push(part);

        i += 1;
    }

    BezierAction {
        parts
    }
}
