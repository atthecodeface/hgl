//a Imports
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use hgl_indexed_vec::VecWithIndex;

use crate::simulation::{
    Clock, ClockArray, ClockIndex, Instance, InstanceHandle, Name, NameFmt, Names, NamespaceStack,
    NsNameFmt, RefInstance, RefMutInstance, SimEdgeMask, SimNsName, SimulationBody,
    SimulationBodyInner, SimulationContents,
};
use crate::traits::{Component, ComponentBuilder, SimHandle, Simulatable};

//a Simulation
//tp Simulation
pub struct Simulation<'s> {
    /// Contents of the simulation that can change during simulation itself
    control: RefCell<SimulationContents<'s>>,

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
        for (i, inst) in self.body.iter_instances().enumerate() {
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
        let control = RefCell::new(SimulationContents::default());
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
    pub fn start(&self, start_running: bool) -> Result<(), String> {
        if self.control.borrow().is_idle() {
            let _failed = self.map_mut_simulatables(|s| s.start(start_running));
            if start_running {
                self.control.borrow_mut().set_running();
            } else {
                self.control.borrow_mut().set_paused();
            }
            Ok(())
        } else {
            Err(format!(
                "Could not start; it was already in state {:?}",
                self.control.borrow().running_state()
            ))
        }
    }

    //mp pause
    pub fn pause(&self) -> Result<(), String> {
        if self.control.borrow().is_paused() {
            Ok(())
        } else if self.control.borrow().is_running() {
            let _failed = self.map_mut_simulatables(|s| s.pause());
            self.control.borrow_mut().set_paused();
            Ok(())
        } else {
            Err(format!(
                "Could not pause; it was already in state {:?}",
                self.control.borrow().running_state()
            ))
        }
    }

    //mp resume
    pub fn resume(&self) -> Result<(), String> {
        if self.control.borrow().is_running() {
            Ok(())
        } else if self.control.borrow().is_paused() {
            let _failed = self.map_mut_simulatables(|s| s.resume());
            self.control.borrow_mut().set_running();
            Ok(())
        } else {
            Err(format!(
                "Could not resume; it was already in state {:?}",
                self.control.borrow().running_state()
            ))
        }
    }

    //mp stop
    pub fn stop(&self) -> Result<(), String> {
        if self.control.borrow().is_running() || self.control.borrow().is_paused() {
            let _failed = self.map_mut_simulatables(|s| s.stop());
            self.control.borrow_mut().set_stopped();
            Ok(())
        } else {
            Err(format!(
                "Could not stop; it was already in state {:?}",
                self.control.borrow().running_state()
            ))
        }
    }

    //mp next_edges
    /// Get the next *system* clock edges to fire
    ///
    /// This moves on time to that for the clock edges
    pub fn next_edges(&self) -> SimEdgeMask {
        self.control.borrow_mut().clocks.next_edges()
    }

    //mp fire_next_edges
    pub fn fire_next_edges(&self) {
        let ie = self.control.borrow_mut().clocks.next_edges();
        let c = self.control.borrow();
        let inst_edges = c.clocks.instance_edges(&ie);
        self.body.fire_next_edges(inst_edges);
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
        if self.body.contains(&full_name) {
            return Err(format!("Duplicate instance name {name}"));
        };
        let handle = {
            let Some(inner) = &mut self.build else {
                panic!("Argh");
            };
            inner.instantiate::<CB, C>(&mut *control, full_name)
        };
        //        self.build.as_mut().unwrap().instances[handle].configure::<C, _>(
        let Some(build) = &self.build else {
            panic!("Must be building");
        };
        build
            .instance(handle)
            .configure::<C, _>(&mut *control, handle, config_fn)?;
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
    fn map_mut_simulatables<F: FnMut(&mut dyn Simulatable)>(&self, f: F) -> bool {
        self.body.map_mut_simulatables(f)
    }

    //mp connect_clock
    pub fn connect_clock(&self, clock: ClockIndex, instance: InstanceHandle, input: usize) {
        self.control
            .borrow_mut()
            .connect_clock(clock, instance, input);
    }
}
