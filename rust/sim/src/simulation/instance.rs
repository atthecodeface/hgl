//a Imports
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

use hgl_indexed_vec::make_index;

use crate::simulation::{
    Name, Names, RefInstance, RefMutInstance, SimNsName, SimStateIndex, Simulation,
    SimulationContents, StateDesc,
};
use crate::traits::{Component, Simulatable};
use crate::values::fmt;

//a InstanceHandle
//tp InstanceHandle
make_index!(InstanceHandle, usize);

//a Instance
//tp Instance
/// Toplevel instance of a component
///
///
/// The underlying `Simulatable` may have some of its code executed
/// from within a work pool thread, and different instances will be
/// being executed (and hence mutated) from different work pool
/// threads.
///
/// Hence the `Simulatable` is held in a RefCell
///
/// Some `Simulatable` instances (such as virtual machines) may run in
/// separate threads entirely; in this case the `Simulatable` is
/// merely an interface abstraction to the other threads, and it must
/// contain the appropriate concurrent execution handling required to
/// interact with the simulatable.
pub struct Instance {
    name: SimNsName,

    /// Wrapped [Simulatable] type, that is usually a [Component],
    /// which can be accessed mutably and immutably
    ///
    /// This is wrapped in a [RwLock] so that multiple instances may
    /// be simulated by different threads at the same time
    ///
    /// If a component model has a thread of execution it should idle
    /// until it receives a message from the engine thread (due to a
    /// 'clock', 'propagate', or similar call; such calls
    simulatable: RefCell<Box<dyn Simulatable + 'static>>,

    /// The description of exposed state for the `Simulatable`; this
    /// may only be interrogated if the simulatable has been paused
    state_map: RefCell<HashMap<Name, StateDesc>>,
}

//ip Debug for Instance
impl std::fmt::Debug for Instance {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Instance[{:?}]", self.name)
    }
}

//ip Instance
impl Instance {
    //cp new
    pub fn new<S: Simulatable + 'static>(name: SimNsName, s: S) -> Self {
        let s: Box<dyn Simulatable + 'static> = Box::new(s);
        let simulatable = RefCell::new(s);
        let state_map = RefCell::new(HashMap::default());
        Self {
            name,
            simulatable,
            state_map,
        }
    }

    //ap name
    pub fn name(&self) -> SimNsName {
        self.name
    }

    //ap borrow_mut
    /// Borrow the instance mutably as the correct [Component] type
    pub fn borrow_mut<C: Component + 'static>(&self) -> Option<RefMutInstance<'_, C>> {
        RefMutInstance::try_ref_mut_instance(&self.simulatable)
    }

    //ap borrow
    /// Borrow the instance mutably as the correct [Component] type
    pub fn borrow<C: Component + 'static>(&self) -> Option<RefInstance<'_, C>> {
        RefInstance::try_ref_instance(&self.simulatable)
    }

    //ap borrow_sim_mut
    /// Borrow the instance mutably as a Simulatable
    pub fn borrow_sim_mut(&self) -> Option<RefMut<'_, Box<dyn Simulatable>>> {
        match self.simulatable.try_borrow_mut() {
            Ok(l) => Some(l),
            Err(_) => None,
        }
    }

    //ap borrow_sim
    /// Borrow the instance immutably as a Simulatable
    pub fn borrow_sim(&self) -> Option<Ref<'_, Box<dyn Simulatable>>> {
        let l = self.simulatable.try_borrow();
        match l {
            Ok(l) => Some(l),
            Err(_) => None,
        }
    }

    //mp configure
    pub fn configure<C: Component, F: FnOnce() -> <C as Component>::Config>(
        &self,
        control: &mut SimulationContents,
        handle: InstanceHandle,
        config_fn: F,
    ) -> Result<(), String> {
        let mut component = self.borrow_mut::<C>().unwrap();
        component.configure(control, handle, config_fn())?;
        for i in 0..usize::MAX {
            let sdi = i.into();
            let Some(port_info) = component.state_info(sdi) else {
                break;
            };
            let name = control.add_name(port_info.name());
            let port = StateDesc::new(sdi, &port_info, None);
            self.state_map.borrow_mut().insert(name, port);
        }
        Ok(())
    }

    //mp state_index
    pub fn state_index(&self, name: Name) -> Option<SimStateIndex> {
        self.state_map
            .borrow()
            .get(&name)
            .map(|sd| sd.state_index())
    }

    //mp fmt_full
    pub fn fmt_full(
        &self,
        fmt: &mut std::fmt::Formatter,
        names: &Names,
        include_values: bool,
    ) -> Result<(), std::fmt::Error> {
        fmt.write_str("Instance['")?;
        names.fmt_ns_name(fmt, self.name())?;
        fmt.write_str("': ")?;
        for (n, p) in self.state_map.borrow().iter() {
            names.fmt_name(fmt, *n)?;
            if include_values {
                if let Ok(s) = self.simulatable.try_borrow() {
                    fmt.write_str("=")?;
                    if let Some(x) = s.try_state_data(p.state_index()) {
                        x.sim_value().fmt_with(fmt, fmt::FULL)?;
                    }
                }
            }
            fmt.write_str(", ")?;
        }
        fmt.write_str("]")
    }
}
