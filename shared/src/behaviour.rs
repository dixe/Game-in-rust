use crate::base_entity::*;

pub trait Behaviour {

    fn execute(&self, entity: &mut BaseEntity);

    fn finished(&self, entity: &BaseEntity) -> bool;
}
