use nalgebra as na;
use crate::base_entity::*;

pub trait Ai {
    fn run(&self, entity: &mut BaseEntity);
}
