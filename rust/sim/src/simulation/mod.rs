//a Modules
mod clock;
mod instance;
mod names;
mod simulation;
mod traits;

//a Exports
pub use clock::ClockArray;
pub use instance::{Instance, RefInstance, RefMutInstance};
pub use names::{FullName, Names};
pub use simulation::Simulation;
pub use traits::{Component, ComponentBuilder, Simulatable};
pub use traits::{SimHandle, SimRegister};

//a Types
//tp EdgeUse
pub struct EdgeUse {}

//tp SimReset
pub enum SimReset {
    Restart,
}
