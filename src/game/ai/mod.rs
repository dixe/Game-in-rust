mod ai;

pub use self::ai::{run_ais, Ai};

mod behaviour;
pub use self::behaviour::{Behaviour, IdleBehaviour, WalkToBehaviour};

mod behaviour_functions;
