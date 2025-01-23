pub(crate) mod types;

pub mod prelude;
mod traits;
pub(crate) mod utils;
pub use traits::{SimArray, SimBit, SimBv, SimStruct, SimValue};

mod simulation;
pub use simulation::SimHandle;
pub use simulation::SimRegister;
pub use simulation::{Component, ComponentBuilder, Simulatable};
pub use simulation::{EdgeUse, FullName, Simulation};
