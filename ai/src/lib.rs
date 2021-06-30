use shared::*;
use nalgebra as na;
use crate::ais::*;

mod ais;

mod behaviours;

#[no_mangle]
pub extern "Rust" fn regular_enemy_ai() -> Box<dyn Ai<shared::RegularEnemyState>> {
    Box::new(RegularEnemyAi::new())
}
