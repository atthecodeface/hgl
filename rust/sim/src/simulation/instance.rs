//a Imports
use std::marker::PhantomData;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::simulation::{Component, Simulatable};

//a Instance
//tp Instance
/// Toplevel instance of a component
///
/// This probably needs to have some Sync wrapper so that different
/// Instance's can be clocked simultaneously
pub struct Instance {
    //tp simulatable
    /// Wrapped [Simulatable] type, that is usually a [Component],
    /// which can be accessed mutably and immutably
    ///
    /// This is wrapped in a [RwLock] so that multiple instances may
    /// be simulated by different threads at the same time
    simulatable: RwLock<Box<dyn Simulatable + 'static>>,
}

//ip Instance
impl Instance {
    //cp new
    pub fn new<S: Simulatable + 'static>(s: S) -> Self {
        let s: Box<dyn Simulatable + 'static> = Box::new(s);
        let simulatable = RwLock::new(s);
        Self { simulatable }
    }

    //ap borrow_mut
    /// Borrow the instance mutably as the correct [Component] type
    pub fn borrow_mut<'a, C: Component + 'static>(&'a self) -> Option<RefMutInstance<'a, C>> {
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
    pub fn borrow<'a, C: Component + 'static>(&'a self) -> RefInstance<'a, C> {
        let l = self.simulatable.read().unwrap();
        RefInstance {
            l,
            phantom: PhantomData,
        }
    }
}

//a RefMutInstance
//tp RefMutInstance
pub struct RefMutInstance<'a, C: Component + 'static> {
    l: RwLockWriteGuard<'a, Box<dyn Simulatable + 'static>>,
    phantom: PhantomData<&'a C>,
}

//tp Deref for RefMutInstance
impl<'a, C: Component + 'static> std::ops::Deref for RefMutInstance<'a, C> {
    type Target = C;
    fn deref(&self) -> &C {
        self.l.as_any().downcast_ref::<C>().unwrap()
    }
}

//tp DerefMut for RefMutInstance
impl<'a, C: Component + 'static> std::ops::DerefMut for RefMutInstance<'a, C> {
    fn deref_mut(&mut self) -> &mut C {
        self.l.as_mut_any().downcast_mut::<C>().unwrap()
    }
}

//a RefInstancep
//tp RefInstance
pub struct RefInstance<'a, C: Component + 'static> {
    l: RwLockReadGuard<'a, Box<dyn Simulatable + 'static>>,
    phantom: PhantomData<&'a C>,
}

//ip Deref for RefInstance
impl<'a, C: Component + 'static> std::ops::Deref for RefInstance<'a, C> {
    type Target = C;
    fn deref(&self) -> &C {
        self.l.as_any().downcast_ref::<C>().unwrap()
    }
}

//ip Deref for RefMutInstance
impl<'a, C: Component + 'static> RefMutInstance<'a, C> {
    ///  Borrow the inputs as mutable
    pub fn inputs_mut<'i>(&'i mut self) -> C::InputsMut<'i> {
        self.l
            .as_mut_any()
            .downcast_mut::<C>()
            .unwrap()
            .inputs_mut()
    }

    /// Borrow the inputs as immutable
    pub fn inputs<'i>(&'i self) -> C::Inputs<'i> {
        self.l.as_any().downcast_ref::<C>().unwrap().inputs()
    }

    /// Borrow the outputs as immutable
    pub fn outputs<'i>(&'i self) -> C::Outputs<'i> {
        self.l.as_any().downcast_ref::<C>().unwrap().outputs()
    }
}

//ip Deref for RefInstance
impl<'a, C: Component + 'static> RefInstance<'a, C> {
    /// Borrow the inputs as immutable
    pub fn inputs<'i>(&'i self) -> C::Inputs<'i> {
        self.l.as_any().downcast_ref::<C>().unwrap().inputs()
    }

    /// Borrow the outputs as immutable
    pub fn outputs<'i>(&'i self) -> C::Outputs<'i> {
        self.l.as_any().downcast_ref::<C>().unwrap().outputs()
    }
}
