//a Modules
mod clock;
mod edge_mask;
mod instance;
mod instance_ref;
mod names;
mod port;
mod simulation;

//a Exports
pub use clock::{Clock, ClockArray, ClockIndex};
pub use edge_mask::SimEdgeMask;
pub use instance::{Instance, InstanceHandle};
pub use instance_ref::{RefInstance, RefMutInstance};
pub use names::{Name, NameFmt, Names, NamespaceStack, NsNameFmt, SimNsName};
pub use port::{SimStateIndex, SimStateInfo, StateDesc};
pub use simulation::{Simulation, SimulationBody, SimulationControl};

//a Types
//tp SimReset
pub enum SimReset {
    Restart,
}
