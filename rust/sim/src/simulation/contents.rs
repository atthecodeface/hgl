//a Imports
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use hgl_indexed_vec::VecWithIndex;

use crate::simulation::{
    Clock, ClockArray, ClockIndex, Instance, InstanceHandle, Name, NameFmt, Names, NamespaceStack,
    NsNameFmt, RefInstance, RefMutInstance, SimEdgeMask, SimNsName, SimulationContents,
};
use crate::traits::{Component, ComponentBuilder, SimHandle, Simulatable};

//a SimulationBodyInner
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

    //ap contains
    /// Iterate through the instances
    pub fn contains(&self, name: &SimNsName) -> bool {
        self.instances.contains(name)
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
        control: &mut SimulationContents,
        full_name: SimNsName,
    ) -> InstanceHandle {
        let component = CB::instantiate(control, full_name);
        let instance = Instance::new(full_name, component);
        self.instances.insert(full_name, |_| instance).unwrap()
    }

    //ap map_mut_simulatables
    /// Iterate through the instances
    pub fn map_mut_simulatables<F: FnMut(&mut dyn Simulatable)>(&self, mut f: F) -> bool {
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

//a SimulationBody - sharable SimulationBodyInner
//tp SimulationBody
pub struct SimulationBody<'s> {
    /// Instances which can be individually executed by separate
    /// threads
    inner: Arc<SimulationBodyInner<'s>>,
}

//ip Deref for SimulationBody
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
