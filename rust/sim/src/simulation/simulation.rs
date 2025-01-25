//a Imports
use std::cell::RefCell;

use crate::simulation::{
    Clock, ClockArray, ClockIndex, Instance, InstanceArray, InstanceHandle, Name, Names,
    NamespaceStack, RefInstance, RefMutInstance, SimNsName,
};
use crate::traits::{Component, ComponentBuilder, SimHandle, SimRegister};

//a SimulationControl
#[derive(Default)]
struct SimulationControl {
    /// Names and namespaces in the simulation
    names: Names,
    /// Current namespace stack
    namespace_stack: NamespaceStack,
}

//ip SimHandle for InstanceHandle
impl SimHandle for InstanceHandle {}

//a Simulation
//tp Simulation
pub struct Simulation {
    /// Clocks used in the simulation
    clocks: ClockArray,

    /// Control of the simulation that can change during simulation itself
    control: RefCell<SimulationControl>,

    /// Instances which can be individually executed by separate
    /// threads
    instances: InstanceArray,
}

//ip Debug for Simulation
impl std::fmt::Debug for Simulation {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Simulation[clocks:[")?;
        for (i, clk) in self.iter_clocks().enumerate() {
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

//ip Simulation
impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulation {
    //cp new
    /// Create a new simulation
    pub fn new() -> Self {
        let clocks = ClockArray::default();
        let control = RefCell::new(SimulationControl::default());
        let instances = InstanceArray::default();
        Self {
            clocks,
            control,
            instances,
        }
    }

    //mp prepare_simulation
    pub fn prepare_simulation(&mut self) {
        self.clocks.derive_schedule();
    }

    //mp next_edges
    pub fn next_edges(&mut self) -> (usize, usize) {
        self.clocks.next_edges()
    }

    //mp time
    pub fn time(&self) -> usize {
        self.clocks.time()
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
            .map_err(|_e| format!("Duplicate name {name} when trying to create clock"))?;
        Ok(self
            .clocks
            .add_clock(full_name, delay, period, negedge_offset))
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
            .map_err(|_e| format!("Duplicate name {name} when trying to instantiate module"))?;
        drop(control);
        let component = CB::instantiate(self, full_name);
        let instance = Instance::new(full_name, component);
        let handle = self.instances.add(full_name, instance);
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

    //ap iter_clocks
    /// Iterate through the clocks
    pub fn iter_clocks(&self) -> impl std::iter::Iterator<Item = &Clock> {
        self.clocks.into_iter()
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
}

//ip SimRegister for Simulation
impl SimRegister for Simulation {
    type Handle = InstanceHandle;

    fn register_input_edge(
        &self,
        _handle: Self::Handle,
        _input: usize,
        _posedge: bool,
        _negedge: bool,
    ) {
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
