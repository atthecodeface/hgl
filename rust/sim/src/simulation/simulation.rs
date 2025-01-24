//a Imports
use crate::simulation::{ClockArray, FullName, Instance, RefInstance, RefMutInstance};
use crate::simulation::{Component, ComponentBuilder, SimHandle, SimRegister};

//a SimulationControl
struct SimulationControl {
    namespace: (),
}

//a InstanceHandle
//tp InstanceHandle
#[derive(Debug, Clone, Copy)]
pub struct InstanceHandle(usize);

//ip InstanceHandle
impl InstanceHandle {
    fn new(n: usize) -> Self {
        Self(n)
    }
}

//ip SimHandle for InstanceHandle
impl SimHandle for InstanceHandle {}

//a Simulation
//tp Simulation
pub struct Simulation {
    control: SimulationControl,
    clocks: ClockArray,
    instances: Vec<Instance>,
}

//ip Simulation
impl Simulation {
    //cp new
    /// Create a new simulation
    pub fn new() -> Self {
        let namespace = ();
        let clocks = ClockArray::default();
        let control = SimulationControl { namespace };
        let instances = vec![];
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
    ) -> usize {
        self.clocks.add_clock(name, delay, period, negedge_offset)
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
        let full_name = FullName::new(self.control.namespace, name)?;
        let component = CB::instantiate(self, &full_name);
        let instance = Instance::new(component);
        let handle = InstanceHandle::new(self.instances.len());
        self.instances.push(instance);
        let mut instance = self.instances[handle.0].borrow_mut::<C>().unwrap();
        instance.configure(&*self, handle, config_fn())?;
        Ok(handle)
    }

    //ap inst
    /// Get a reference to a component instance given its handle
    pub fn inst<C: Component>(&self, handle: InstanceHandle) -> RefInstance<C> {
        self.instances[handle.0].borrow()
    }

    //ap inst_mut
    /// Get a mutable reference to a component instance given its handle
    pub fn inst_mut<C: Component>(&self, handle: InstanceHandle) -> RefMutInstance<C> {
        self.instances[handle.0].borrow_mut().unwrap()
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
