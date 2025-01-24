//a Modules
mod clock;
mod instance;
mod names;
mod simulation;
mod traits;

//a Exports
pub use clock::{ClockArray, ClockIndex};
pub use instance::{Instance, Port, RefInstance, RefMutInstance};
pub use names::{FullName, FullNameIndex, Name, Names, NamespaceStack};
pub use simulation::{InstanceHandle, Simulation};
pub use traits::{Component, ComponentBuilder, Simulatable};
pub use traits::{SimHandle, SimRegister};

//a Types
//tp EdgeUse
pub struct EdgeUse {}

//tp SimReset
pub enum SimReset {
    Restart,
}
