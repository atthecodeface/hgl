//a Modules
mod clock;
mod instance;
mod names;
mod port;
mod simulation;

//a Exports
pub use clock::{Clock, ClockArray, ClockIndex};
pub use instance::{Instance, InstanceHandle, RefInstance, RefMutInstance};
pub use names::{Name, NameFmt, Names, NamespaceStack, NsNameFmt, SimNsName};
pub use port::{SimStateIndex, SimStateInfo, StateDesc};
pub use simulation::Simulation;

//a Types
//tp SimReset
pub enum SimReset {
    Restart,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct SimEdgeMask(u64);
impl SimEdgeMask {
    pub fn is_none(&self) -> bool {
        self.0 == 0
    }
    pub const fn is_posedge(&self, input: usize) -> bool {
        ((self.0 as usize) >> (input * 2)) & 1 != 0
    }
    pub const fn is_negedge(&self, input: usize) -> bool {
        ((self.0 as usize) >> (input * 2)) & 2 != 0
    }
    pub fn add_posedge(mut self, input: usize) -> Self {
        self.0 |= (1 << (2 * input)) as u64;
        self
    }
    pub fn add_negedge(mut self, input: usize) -> Self {
        self.0 |= (2 << (2 * input)) as u64;
        self
    }
    pub fn set_posedge(&mut self, input: usize) {
        self.0 |= (1 << (2 * input)) as u64;
    }
    pub fn set_negedge(&mut self, input: usize) {
        self.0 |= (2 << (2 * input)) as u64;
    }
}
