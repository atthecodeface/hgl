//a Modules
mod clock;
mod instance;
mod names;
mod port;
mod simulation;

//a Exports
pub use clock::{Clock, ClockArray, ClockIndex};
pub use instance::{Instance, InstanceArray, InstanceHandle, RefInstance, RefMutInstance};
pub use names::{Name, Names, NamespaceStack, SimNsName};
pub use port::{SimStateIndex, SimStateInfo, StateDesc};
pub use simulation::Simulation;

//a Types
//tp EdgeUse
pub struct EdgeUse {}

//tp SimReset
pub enum SimReset {
    Restart,
}
