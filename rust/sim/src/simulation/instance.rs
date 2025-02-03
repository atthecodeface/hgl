//a Imports
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use hgl_indexed_vec::make_index;

use crate::simulation::{Name, Names, SimNsName, SimStateIndex, Simulation, StateDesc};
use crate::traits::{Component, SimBit, SimBv, SimCopyValue, Simulatable};
use crate::values::fmt;

//a InstanceHandle
//tp InstanceHandle
make_index!(InstanceHandle, usize);

//a RefMutInstance
//tp RefMutInstance
pub struct RefMutInstance<'a, C: Component + 'static> {
    l: RwLockWriteGuard<'a, Box<dyn Simulatable + 'static>>,
    phantom: PhantomData<&'a C>,
}

//tp Deref for RefMutInstance
impl<C: Component + 'static> std::ops::Deref for RefMutInstance<'_, C> {
    type Target = C;
    fn deref(&self) -> &C {
        self.l.as_any().downcast_ref::<C>().unwrap()
    }
}

//tp DerefMut for RefMutInstance
impl<C: Component + 'static> std::ops::DerefMut for RefMutInstance<'_, C> {
    fn deref_mut(&mut self) -> &mut C {
        self.l.as_mut_any().downcast_mut::<C>().unwrap()
    }
}

//a RefInstance
//tp RefInstance
pub struct RefInstance<'a, C: Component + 'static> {
    l: RwLockReadGuard<'a, Box<dyn Simulatable + 'static>>,
    phantom: PhantomData<&'a C>,
}

//ip Deref for RefInstance
impl<C: Component + 'static> std::ops::Deref for RefInstance<'_, C> {
    type Target = C;
    fn deref(&self) -> &C {
        self.l.as_any().downcast_ref::<C>().unwrap()
    }
}

//ip Deref for RefMutInstance
impl<C: Component + 'static> RefMutInstance<'_, C> {
    ///  Borrow the inputs as mutable
    pub fn inputs_mut(&mut self) -> C::InputsMut<'_> {
        self.l
            .as_mut_any()
            .downcast_mut::<C>()
            .unwrap()
            .inputs_mut()
    }

    /// Borrow the inputs as immutable
    pub fn inputs(&self) -> C::Inputs<'_> {
        self.l.as_any().downcast_ref::<C>().unwrap().inputs()
    }

    /// Borrow the outputs as immutable
    pub fn outputs(&self) -> C::Outputs<'_> {
        self.l.as_any().downcast_ref::<C>().unwrap().outputs()
    }
}

//ip RefInstance
impl<C: Component + 'static> RefInstance<'_, C> {
    /// Borrow the inputs as immutable
    pub fn inputs(&self) -> C::Inputs<'_> {
        self.l.as_any().downcast_ref::<C>().unwrap().inputs()
    }

    /// Borrow the outputs as immutable
    pub fn outputs(&self) -> C::Outputs<'_> {
        self.l.as_any().downcast_ref::<C>().unwrap().outputs()
    }
    pub fn try_as_t<V: SimCopyValue>(&self, s: SimStateIndex) -> Option<V> {
        self.l.as_any().downcast_ref::<C>().and_then(|c| {
            c.try_state_data(s)
                .and_then(|sd| sd.try_as_t::<V>().copied())
        })
    }
    pub fn as_t<V: SimCopyValue>(&self, s: SimStateIndex) -> V {
        self.try_as_t(s).unwrap()
    }
    pub fn try_as_u64<V: SimBv>(&self, s: SimStateIndex) -> Option<u64> {
        self.l
            .as_any()
            .downcast_ref::<C>()
            .unwrap()
            .try_state_data(s)
            .and_then(|v| v.try_as_u64::<V>())
    }
    pub fn try_as_bool<V>(&self, s: SimStateIndex) -> Option<bool>
    where
        V: SimBit,
        bool: From<V>,
        for<'b> &'b bool: From<&'b V>,
    {
        self.l
            .as_any()
            .downcast_ref::<C>()
            .unwrap()
            .try_state_data(s)
            .and_then(|v| v.try_as_bool::<V>())
    }
}

//a Instance
//tp Instance
/// Toplevel instance of a component
///
/// This probably needs to have some Sync wrapper so that different
/// Instance's can be clocked simultaneously
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
    simulatable: RwLock<Box<dyn Simulatable + 'static>>,

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
        let simulatable = RwLock::new(s);
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
        let l = self.simulatable.try_write();
        match l {
            Ok(l) => Some(RefMutInstance {
                l,
                phantom: PhantomData,
            }),
            Err(_) => None,
        }
    }

    //ap borrow
    /// Borrow the instance immutably as the correct [Component] type
    pub fn borrow<C: Component + 'static>(&self) -> Option<RefInstance<'_, C>> {
        let l = self.simulatable.try_read();
        match l {
            Ok(l) => Some(RefInstance {
                l,
                phantom: PhantomData,
            }),
            Err(_) => None,
        }
    }

    //ap borrow_sim_mut
    /// Borrow the instance mutably as a Simulatable
    pub fn borrow_sim_mut(&self) -> Option<RwLockWriteGuard<'_, Box<dyn Simulatable>>> {
        match self.simulatable.try_write() {
            Ok(l) => Some(l),
            Err(_) => None,
        }
    }

    //ap borrow_sim
    /// Borrow the instance immutably as a Simulatable
    pub fn borrow_sim(&self) -> Option<RwLockReadGuard<'_, Box<dyn Simulatable>>> {
        let l = self.simulatable.try_read();
        match l {
            Ok(l) => Some(l),
            Err(_) => None,
        }
    }

    //mp configure
    pub fn configure<C: Component, F: FnOnce() -> <C as Component>::Config>(
        &self,
        sim: &Simulation,
        handle: InstanceHandle,
        config_fn: F,
    ) -> Result<(), String> {
        let mut component = self.borrow_mut::<C>().unwrap();
        component.configure(sim, handle, config_fn())?;
        for i in 0..usize::MAX {
            let sdi = i.into();
            let Some(port_info) = component.state_info(sdi) else {
                break;
            };
            let name = sim.add_name(port_info.name());
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
                if let Ok(s) = self.simulatable.try_read() {
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
