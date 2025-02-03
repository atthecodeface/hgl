//a Imports
use std::marker::PhantomData;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

use crate::simulation::SimStateIndex;
use crate::traits::{Component, SimBit, SimBv, SimCopyValue, Simulatable};

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

//ip RefMutInstance
impl<'a, C: Component + 'static> RefMutInstance<'a, C> {
    //cp of_lock_guard
    pub fn of_lock_guard(l: RwLockWriteGuard<'a, Box<dyn Simulatable + 'static>>) -> Self {
        Self {
            l,
            phantom: PhantomData,
        }
    }
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

//ip RefInstance
impl<'a, C: Component + 'static> RefInstance<'a, C> {
    //cp of_lock_guard
    pub fn of_lock_guard(l: RwLockReadGuard<'a, Box<dyn Simulatable + 'static>>) -> Self {
        Self {
            l,
            phantom: PhantomData,
        }
    }
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
