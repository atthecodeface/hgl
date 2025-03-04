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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Running {
    #[default]
    Idle,
    Paused,
    Running,
    Stopped,
}
#[derive(Default)]
pub struct SimulationControl<'s> {
    /// Names and namespaces in the simulation
    names: Names<'s>,
    /// Current namespace stack
    namespace_stack: NamespaceStack,
    /// Use of edges by instances
    edge_uses: HashMap<InstanceHandle, Vec<EdgeUse>>,
    /// Clocks used in the simulation
    clocks: ClockArray<'s>,
    /// State of simulation
    running_state: Running,
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
    //mp add_name
    pub fn add_name(&mut self, name: &str) -> Name {
        self.names.add_name(name)
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

//ip SimHandle for InstanceHandle
impl SimHandle for InstanceHandle {}

//a Simulation
//tp SimulationBodyInner
pub struct SimulationBodyInner<'s> {
    /// Instances which can be individually executed by separate
    /// threads
    instances: VecWithIndex<'s, SimNsName, InstanceHandle, Instance>,
}

//ip SimulationBodyInner
impl SimulationBodyInner<'_> {
    //cp new
    /// Create a new simulation
    pub fn new() -> Self {
        let instances = VecWithIndex::default();
        Self { instances }
    }

    //ap iter_instances
    /// Iterate through the instances
    pub fn iter_instances(&self) -> impl std::iter::Iterator<Item = &Instance> {
        self.instances.into_iter()
    }

    //mp fire_next_edges
    pub fn fire_next_edges(&self, inst_edges: &[(InstanceHandle, SimEdgeMask)]) {
        for (inst, edge_mask) in inst_edges {
            self.instances[*inst]
                .borrow_sim_mut()
                .unwrap()
                .clock(*edge_mask);
        }
    }

    //mp instantiate
    /// Instantiate a component in the simulation with a given name,
    /// using the specified [ComponentBuilder]
    ///
    /// After instantiation the 'config_fn' is executed to provide the
    /// configuration for the component
    pub fn instantiate<CB: ComponentBuilder<Build = C>, C: Component>(
        &mut self,
        control: &mut SimulationControl,
        full_name: SimNsName,
    ) -> InstanceHandle {
        let component = CB::instantiate(control, full_name);
        let instance = Instance::new(full_name, component);
        self.instances.insert(full_name, |_| instance).unwrap()
    }

    //ap map_mut_simulatables
    /// Iterate through the instances
    fn map_mut_simulatables<F: FnMut(&mut dyn Simulatable)>(&self, f: &mut F) -> bool {
        let mut mapped_all = true;
        for i in self.iter_instances() {
            use std::ops::DerefMut;
            if let Some(mut s) = i.borrow_sim_mut() {
                f(s.deref_mut().deref_mut())
            } else {
                mapped_all = false;
            }
        }
        mapped_all
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

//tp SimulationBody
pub struct SimulationBody<'s> {
    /// Instances which can be individually executed by separate
    /// threads
    inner: Arc<SimulationBodyInner<'s>>,
}

impl<'s> std::ops::Deref for SimulationBody<'s> {
    type Target = SimulationBodyInner<'s>;
    fn deref(&self) -> &SimulationBodyInner<'s> {
        self.inner.deref()
    }
}

//ip SimulationBody
impl<'s> SimulationBody<'s> {
    //cp new
    /// Create a new simulation
    pub fn new(inner: SimulationBodyInner<'s>) -> Self {
        let inner = Arc::new(inner);
        Self { inner }
    }
    //cp empty
    /// Create a new simulation
    pub fn empty() -> Self {
        Self::new(SimulationBodyInner::new())
    }
    //cp is_empty
    /// Create a new simulation
    pub fn is_empty(&self) -> bool {
        self.inner.instances.is_empty()
    }
}

//ip Clone for SimulationBody
impl<'s> Clone for SimulationBody<'s> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

//tp Simulation
pub struct Simulation<'s> {
    /// Control of the simulation that can change during simulation itself
    control: RefCell<SimulationControl<'s>>,

    build: Option<SimulationBodyInner<'s>>,
    body: SimulationBody<'s>,
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
        for (i, inst) in self.body.inner.iter_instances().enumerate() {
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
impl Simulation<'_> {
    //cp new
    /// Create a new simulation
    pub fn new() -> Self {
        let control = RefCell::new(SimulationControl::default());
        let build = Some(SimulationBodyInner::new());
        let body = SimulationBody::empty();
        Self {
            control,
            body,
            build,
        }
    }

    //mp prepare_simulation
    pub fn prepare_simulation(&mut self) {
        assert!(
            self.build.is_some(),
            "Can only prepare simulation if it was being built"
        );
        assert!(self.body.is_empty(), "Build should be empty if being built");
        self.control.borrow_mut().clocks.derive_schedule();
        self.body = SimulationBody::new(self.build.take().unwrap());
    }

    //mp instances
    pub fn instances(&self) -> SimulationBody {
        assert!(
            self.build.is_none(),
            "Can only get instances after prepare_simulation"
        );
        self.body.clone()
    }

    //mp start
    pub fn start(&self, running: bool) -> Result<(), String> {
        let running_state = self.control.borrow().running_state;
        if running_state == Running::Idle {
            let _failed = self.map_mut_simulatables(&mut |s| s.start(running));
            if running {
                self.control.borrow_mut().running_state = Running::Running;
            } else {
                self.control.borrow_mut().running_state = Running::Paused;
            }
            Ok(())
        } else {
            Err(format!(
                "Could not start; it was already in state {running_state:?}"
            ))
        }
    }

    //mp pause
    pub fn pause(&self) -> Result<(), String> {
        let running_state = self.control.borrow().running_state;
        match running_state {
            Running::Paused => Ok(()),
            Running::Running => {
                let _failed = self.map_mut_simulatables(&mut |s| s.pause());
                Ok(())
            }
            _ => Err(format!(
                "Could not pause; it was already in state {running_state:?}"
            )),
        }
    }

    //mp resume
    pub fn resume(&self) -> Result<(), String> {
        let running_state = self.control.borrow().running_state;
        match running_state {
            Running::Running => Ok(()),
            Running::Paused => {
                let _failed = self.map_mut_simulatables(&mut |s| s.resume());
                Ok(())
            }
            _ => Err(format!(
                "Could not resume; it was already in state {running_state:?}"
            )),
        }
    }

    //mp stop
    pub fn stop(&self) -> Result<(), String> {
        let running_state = self.control.borrow().running_state;
        match running_state {
            Running::Running | Running::Paused => {
                let _failed = self.map_mut_simulatables(&mut |s| s.stop());
                Ok(())
            }
            _ => Err(format!(
                "Could not stop; it was already in state {running_state:?}"
            )),
        }
    }

    //mp next_edges
    pub fn next_edges(&self) -> (usize, usize) {
        self.control.borrow_mut().clocks.next_edges()
    }

    //mp fire_next_edges
    pub fn fire_next_edges(&self) {
        let ie = self.control.borrow_mut().clocks.next_edges();
        let c = self.control.borrow();
        let inst_edges = c.clocks.instance_edges(&ie);
        self.body.inner.fire_next_edges(inst_edges);
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
        if self.body.inner.instances.contains(&full_name) {
            return Err(format!("Duplicate instance name {name}"));
        };
        let handle = {
            let Some(inner) = &mut self.build else {
                panic!("Argh");
            };
            inner.instantiate::<CB, C>(&mut *control, full_name)
        };
        self.build.as_mut().unwrap().instances[handle].configure::<C, _>(
            &mut *control,
            handle,
            config_fn,
        )?;
        Ok(handle)
    }

    //mp add_name
    pub fn add_name(&self, name: &str) -> Name {
        self.control.borrow_mut().add_name(name)
    }

    //mp find_name
    pub fn find_name(&self, name: &str) -> Option<Name> {
        self.control.borrow().names.find_name(name)
    }

    //mp map_mut_simulatables
    /// Iterate through the instances
    fn map_mut_simulatables<F: FnMut(&mut dyn Simulatable)>(&self, f: &mut F) -> bool {
        self.body.inner.map_mut_simulatables(f)
    }

    //mp connect_clock
    pub fn connect_clock(&self, clock: ClockIndex, instance: InstanceHandle, input: usize) {
        self.control
            .borrow_mut()
            .connect_clock(clock, instance, input);
    }
}

//ip SimRegister for SimulationControl
impl SimRegister for SimulationControl<'_> {
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
