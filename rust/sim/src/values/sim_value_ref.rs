//a Imports
use crate::traits::{SimBit, SimBv, SimCopyValue, SimValueObject};

//a SimValueRef
//tp SimValueRef
/// An immutable reference to a simulation value, usually belonging to
/// an instance of a component, that can be accessed as 'state data'
/// from a user of a Simulation
///
/// It holds a reference to the value, which must provide
/// [SimValueObject]; that trait provide an 'as_any' method, so a
/// SimValueRef can have its value borrowed as a dyn Any and downcast
/// appropriately if required.
///
/// However, to make access simpler to the underlying type, 'as_t' and
/// 'try_as_t' methods attempt to downcast directly to a type.
#[derive(Debug)]
pub struct SimValueRef<'a> {
    value: &'a (dyn SimValueObject),
}

impl<'a> SimValueRef<'a> {
    pub fn of(value: &'a dyn SimValueObject) -> Self {
        Self { value }
    }
    pub fn sim_value(&self) -> &dyn SimValueObject {
        self.value
    }
    pub fn value<V: SimCopyValue>(&self) -> Option<V> {
        self.value.as_any().downcast_ref::<V>().copied()
    }
    pub fn try_as_t<V: 'static>(&self) -> Option<&V> {
        self.value.as_any().downcast_ref::<V>()
    }
    pub fn as_t<V: 'static>(&self) -> &V {
        self.try_as_t().unwrap()
    }
    pub fn try_as_u64<V: SimBv>(&self) -> Option<u64> {
        self.try_as_t().and_then(|v: &V| v.try_as_u64())
    }
    pub fn try_as_bool<V>(&self) -> Option<bool>
    where
        V: SimBit,
        bool: From<V>,
        for<'b> &'b bool: From<&'b V>,
    {
        self.try_as_t().map(|v: &V| v.is_true())
    }
    pub fn as_any(&self) -> &dyn std::any::Any {
        self.value.as_any()
    }
}

#[derive(Debug)]
pub struct SimValueRefMut<'a> {
    value: &'a mut (dyn SimValueObject),
}

impl<'a> SimValueRefMut<'a> {
    pub fn of(value: &'a mut dyn SimValueObject) -> Self {
        Self { value }
    }
    pub fn try_copy_from(&mut self, other: &SimValueRef) -> bool {
        if other.as_any().type_id() != self.as_any().type_id() {
            false
        } else {
            let Some(size) = self.value.try_as_u8s_mut() else {
                return false;
            };
            let Some(osize) = other.value.try_as_u8s() else {
                return false;
            };
            assert_eq!(
                size.len(),
                osize.len(),
                "Sizes of port data to copy must match"
            );
            size.copy_from_slice(osize);
            true
        }
    }
    pub fn set_u8s(&mut self, data: &[u8]) -> bool {
        let Some(size) = self.value.try_as_u8s_mut() else {
            return false;
        };
        if data.len() != size.len() {
            dbg!(data, size);
            return false;
        }
        size.copy_from_slice(data);
        true
    }
    pub fn sim_value(&self) -> &dyn SimValueObject {
        self.value
    }
    pub fn as_any(&self) -> &dyn std::any::Any {
        self.value.as_any()
    }
    pub fn value<V: SimCopyValue>(&self) -> Option<V> {
        self.value.as_any().downcast_ref::<V>().copied()
    }
    pub fn as_t<V: SimCopyValue>(&self) -> &V {
        self.value.as_any().downcast_ref::<V>().unwrap()
    }
    pub fn try_as_u64<V: SimBv>(&self) -> Option<u64> {
        self.value
            .as_any()
            .downcast_ref::<V>()
            .and_then(|v| v.try_as_u64())
    }
    pub fn try_as_bool<V>(&self) -> Option<bool>
    where
        V: SimBit,
        bool: From<V>,
        for<'b> &'b bool: From<&'b V>,
    {
        self.value.as_any().downcast_ref::<V>().map(|v| v.is_true())
    }
    // pub fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
    // self.value.as_any()
    // }
}
