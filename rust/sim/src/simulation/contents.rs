//a Imports
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use hgl_indexed_vec::VecWithIndex;

use crate::simulation::{
    Clock, ClockArray, ClockIndex, Instance, InstanceHandle, Name, NameFmt, Names, NamespaceStack,
    NsNameFmt, RefInstance, RefMutInstance, SimEdgeMask, SimNsName,
};
use crate::traits::{Component, ComponentBuilder, SimHandle, SimRegister, Simulatable};

//a SimulationContents
//tp EdgeUse
#[derive(Default, Debug)]
pub struct EdgeUse {
    instance: InstanceHandle,
    input: usize,
    posedge: bool,
    negedge: bool,
}

//tp Running
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Running {
    #[default]
    Idle,
    Paused,
    Running,
    Stopped,
}

//tp SimulationContents
#[derive(Default)]
pub struct SimulationContents<'s> {
    /// Names and namespaces in the simulation
    pub names: Names<'s>,
    /// Current namespace stack
    pub namespace_stack: NamespaceStack,
    /// Use of edges by instances
    edge_uses: HashMap<InstanceHandle, Vec<EdgeUse>>,
    /// Clocks used in the simulation
    pub clocks: ClockArray<'s>,
    /// State of simulation
    running_state: Running,
}

//ip SimulationContents
impl SimulationContents<'_> {
    //ap ns_name_fmt
    pub fn ns_name_fmt(&self, name: SimNsName) -> NsNameFmt {
        self.names.ns_name_fmt(name)
    }
    //ap name_fmt
    pub fn name_fmt(&self, name: Name) -> NameFmt {
        self.names.name_fmt(name)
    }
    //mp add_name
    pub fn add_name(&mut self, name: &str) -> Name {
        self.names.add_name(name)
    }

    //ap running_state
    pub fn running_state(&self) -> Running {
        self.running_state
    }

    //ap is_idle
    pub fn is_idle(&self) -> bool {
        self.running_state == Running::Idle
    }

    //ap is_running
    pub fn is_running(&self) -> bool {
        self.running_state == Running::Running
    }

    //ap is_paused
    pub fn is_paused(&self) -> bool {
        self.running_state == Running::Paused
    }

    //ap is_stopped
    pub fn is_stopped(&self) -> bool {
        self.running_state == Running::Stopped
    }

    //ap set_running
    pub fn set_running(&mut self) {
        self.running_state = Running::Running;
    }

    //ap set_paused
    pub fn set_paused(&mut self) {
        self.running_state = Running::Paused;
    }

    //ap set_stopped
    pub fn set_stopped(&mut self) {
        self.running_state = Running::Stopped;
    }

    //ap iter_clocks
    /// Iterate through the clocks
    pub fn iter_clocks(&self) -> impl std::iter::Iterator<Item = &Clock> {
        self.clocks.iter()
    }

    pub fn register_input_use(
        &mut self,
        instance: InstanceHandle,
        input: usize,
        posedge: bool,
        negedge: bool,
    ) {
        self.edge_uses.entry(instance).or_default().push(EdgeUse {
            instance,
            input,
            posedge,
            negedge,
        });
    }
    pub fn connect_clock(&mut self, clock: ClockIndex, instance: InstanceHandle, input: usize) {
        let Some(edge_uses) = self.edge_uses.get(&instance) else {
            return;
        };
        for e in edge_uses.iter() {
            if e.input == input {
                if e.posedge {
                    self.clocks.edge_used_by(clock, instance, input, true);
                    // clock posedge used by instance and when its posedge fires must set SimEdgeMask.posedge(input) for the instance
                }
                if e.negedge {
                    // clock negedge used by instance and when its negedge fires must set SimEdgeMask.negedge(input) for the instance
                    self.clocks.edge_used_by(clock, instance, input, false);
                }
            }
        }
    }
}

//ip SimRegister for SimulationContents
impl SimRegister for SimulationContents<'_> {
    type Handle = InstanceHandle;

    fn register_input_edge(
        &mut self,
        handle: Self::Handle,
        input: usize,
        posedge: bool,
        negedge: bool,
    ) {
        self.register_input_use(handle, input, posedge, negedge);
    }
    fn comb_path(
        &mut self,
        _handle: Self::Handle,
        _outputs_ib: &[u8],
        _inputs_ib: &[u8],
        _outputs_ia: &[u8],
    ) {
    }
}
