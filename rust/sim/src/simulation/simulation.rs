//a Imports
use std::cell::RefCell;
use std::collections::HashMap;

use hgl_indexed_vec::VecWithIndex;

use crate::simulation::{
    Clock, ClockArray, ClockIndex, Instance, InstanceHandle, Name, NameFmt, Names, NamespaceStack,
    NsNameFmt, RefInstance, RefMutInstance, SimEdgeMask, SimNsName,
};
use crate::traits::{Component, ComponentBuilder, SimHandle, SimRegister};

//a SimulationControl
//tp EdgeUse
#[derive(Default, Debug)]
pub struct EdgeUse {
    instance: InstanceHandle,
    input: usize,
    posedge: bool,
    negedge: bool,
}

//tp SimulationControl
#[derive(Default)]
struct SimulationControl<'s> {
    /// Names and namespaces in the simulation
    names: Names<'s>,
    /// Current namespace stack
    namespace_stack: NamespaceStack,
    /// Use of edges by instances
    edge_uses: HashMap<InstanceHandle, Vec<EdgeUse>>,
    /// Clocks used in the simulation
    clocks: ClockArray<'s>,
}

//ip SimulationControl
impl SimulationControl<'_> {
    //ap ns_name_fmt
    pub fn ns_name_fmt(&self, name: SimNsName) -> NsNameFmt {
        self.names.ns_name_fmt(name)
    }
    //ap name_fmt
    pub fn name_fmt(&self, name: Name) -> NameFmt {
        self.names.name_fmt(name)
    }
    //ap iter_clocks
    /// Iterate through the clocks
    pub fn iter_clocks(&self) -> impl std::iter::Iterator<Item = &Clock> {
        self.clocks.into_iter()
    }

    pub fn register_input_use(
        &mut self,
        instance: InstanceHandle,
        input: usize,
        posedge: bool,
        negedge: bool,
    ) {
        self.edge_uses
            .entry(instance)
            .or_insert_with(|| vec![])
            .push(EdgeUse {
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

//ip SimHandle for InstanceHandle
impl SimHandle for InstanceHandle {}

//a Simulation
//tp Simulation
pub struct Simulation<'s> {
    /// Control of the simulation that can change during simulation itself
    control: RefCell<SimulationControl<'s>>,

    /// Instances which can be individually executed by separate
    /// threads
    instances: VecWithIndex<'s, SimNsName, InstanceHandle, Instance>,
}

//ip Debug for Simulation
impl std::fmt::Debug for Simulation<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Simulation[clocks:[")?;
        for (i, clk) in self.control.borrow().iter_clocks().enumerate() {
            if i > 0 {
                fmt.write_str(", ")?;
            }
            fmt.write_str("'")?;
            self.control.borrow().names.fmt_ns_name(fmt, clk.name())?;
            fmt.write_str("'")?;
        }
        write!(fmt, "], instances:[")?;
        for (i, inst) in self.iter_instances().enumerate() {
            if i > 0 {
                fmt.write_str(", ")?;
            }
            fmt.write_str("'")?;
            self.control.borrow().names.fmt_ns_name(fmt, inst.name())?;

            inst.fmt_full(fmt, &self.control.borrow().names, true)?;

            fmt.write_str("'")?;
        }
        write!(fmt, "]]")
    }
}

//ip Default for Simulation
impl Default for Simulation<'_> {
    fn default() -> Self {
        Self::new()
    }
}

//ip Simulation
impl Simulation<'_> {
    //cp new
    /// Create a new simulation
    pub fn new() -> Self {
        let control = RefCell::new(SimulationControl::default());
        let instances = VecWithIndex::default();
        Self { control, instances }
    }

    //mp prepare_simulation
    pub fn prepare_simulation(&self) {
        self.control.borrow_mut().clocks.derive_schedule();
    }

    //mp next_edges
    pub fn next_edges(&self) -> (usize, usize) {
        self.control.borrow_mut().clocks.next_edges()
    }

    //mp fire_next_edges
    pub fn fire_next_edges(&self) {
        let ie = self.control.borrow_mut().clocks.next_edges();
        let c = self.control.borrow();
        for (inst, edge_mask) in c.clocks.instance_edges(&ie).iter() {
            self.instances[*inst]
                .borrow_sim_mut()
                .unwrap()
                .clock(*edge_mask);
        }
    }

    //mp time
    pub fn time(&self) -> usize {
        self.control.borrow().clocks.time()
    }

    //mp add_clock
    /// Add a clock by name, within the current namespace
    ///
    /// There is a delay until the first posedge clock, then it has a
    /// posedge repeatedly after every 'period'; the negedge_offset
    /// should be less than period, and is the delay from the posedge
    /// to the negedge; a value of 0 means a negedge is not simulated
    pub fn add_clock(
        &mut self,
        name: &str,
        delay: usize,
        period: usize,
        negedge_offset: usize,
    ) -> Result<ClockIndex, String> {
        let mut control = self.control.borrow_mut();
        let namespace = control.namespace_stack.top();
        let full_name = control
            .names
            .insert_full_name(namespace, name)
            .map_err(|ns_name| {
                format!(
                    "Duplicate name {} when trying to create clock",
                    control.ns_name_fmt(ns_name)
                )
            })?;
        control
            .clocks
            .add_clock(full_name, delay, period, negedge_offset)
    }

    //mp find_clock
    /// Find a clock by name
    pub fn find_clock(&self, name: SimNsName) -> Option<ClockIndex> {
        None
        // self
        // .clocks
        // .add_clock(full_name, delay, period, negedge_offset))
    }

    //mp instantiate
    /// Instantiate a component in the simulation with a given name,
    /// using the specified [ComponentBuilder]
    ///
    /// After instantiation the 'config_fn' is executed to provide the
    /// configuration for the component
    pub fn instantiate<
        CB: ComponentBuilder<Build = C>,
        C: Component,
        F: FnOnce() -> <C as Component>::Config,
    >(
        &mut self,
        name: &str,
        config_fn: F,
    ) -> Result<InstanceHandle, String> {
        let mut control = self.control.borrow_mut();
        let namespace = control.namespace_stack.top();
        let full_name = control
            .names
            .insert_full_name(namespace, name)
            .map_err(|ns_name| {
                format!(
                    "Duplicate name {} when trying to instantiate module",
                    control.ns_name_fmt(ns_name)
                )
            })?;
        drop(control);
        let component = CB::instantiate(self, full_name);
        let instance = Instance::new(full_name, component);
        let handle = self
            .instances
            .insert(full_name, |_| instance)
            .map_err(|_e| {
                format!(
                    "Instance with name {} already exists",
                    self.control.borrow().ns_name_fmt(full_name)
                )
            })?;
        self.instances[handle].configure::<C, _>(self, handle, config_fn)?;
        Ok(handle)
    }

    //mp add_name
    pub fn add_name(&self, name: &str) -> Name {
        self.control.borrow_mut().names.add_name(name)
    }

    //mp find_name
    pub fn find_name(&self, name: &str) -> Option<Name> {
        self.control.borrow().names.find_name(name)
    }

    //ap iter_instances
    /// Iterate through the instances
    pub fn iter_instances(&self) -> impl std::iter::Iterator<Item = &Instance> {
        self.instances.into_iter()
    }

    //ap inst
    /// Get a reference to a component instance given its handle
    pub fn inst<C: Component>(&self, handle: InstanceHandle) -> RefInstance<C> {
        self.instances[handle].borrow().unwrap()
    }

    //ap inst_mut
    /// Get a mutable reference to a component instance given its handle
    pub fn inst_mut<C: Component>(&self, handle: InstanceHandle) -> RefMutInstance<C> {
        self.instances[handle].borrow_mut().unwrap()
    }
    //ap instance
    /// Get the Instance
    pub fn instance(&self, handle: InstanceHandle) -> &Instance {
        &self.instances[handle]
    }

    //mp connect_clock
    pub fn connect_clock(&self, clock: ClockIndex, instance: InstanceHandle, input: usize) {
        self.control
            .borrow_mut()
            .connect_clock(clock, instance, input);
    }
}

//ip SimRegister for Simulation
impl SimRegister for Simulation<'_> {
    type Handle = InstanceHandle;

    fn register_input_edge(
        &self,
        handle: Self::Handle,
        input: usize,
        posedge: bool,
        negedge: bool,
    ) {
        let mut control = self.control.borrow_mut();
        control.register_input_use(handle, input, posedge, negedge);
    }
    fn comb_path(
        &self,
        _handle: Self::Handle,
        _outputs_ib: &[u8],
        _inputs_ib: &[u8],
        _outputs_ia: &[u8],
    ) {
    }
}
