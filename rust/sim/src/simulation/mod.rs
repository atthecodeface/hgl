//a Modules
mod clock;
mod instance;
mod names;
mod port;
mod simulation;
mod traits;

//a Exports
pub use clock::{ClockArray, ClockIndex};
pub use instance::{InstanceArray, InstanceHandle, RefInstance, RefMutInstance};
pub use names::{Name, Names, NamespaceStack, SimNsName};
pub use port::{Port, PortData, PortDataMut, PortInfo};
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
