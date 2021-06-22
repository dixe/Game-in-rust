use shared::*;
use nalgebra as na;
use crate::ais::*;

mod ais;
#[no_mangle]
pub extern "Rust" fn empty_ai() -> Box<dyn Ai> {
    Box::new(EmptyAi::new())
}
