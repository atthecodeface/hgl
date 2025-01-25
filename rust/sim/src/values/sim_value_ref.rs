//a Imports
use crate::traits::{SimValue, SimValueObject};

//a SimValueRef
//tp SimValueRef
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
    pub fn value<V: SimValue>(&self) -> Option<V> {
        self.value.as_any().downcast_ref::<V>().copied()
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
    pub fn value<V: SimValue>(&self) -> Option<V> {
        self.value.as_any().downcast_ref::<V>().copied()
    }
    // pub fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
    // self.value.as_any()
    // }
}
