//a Imports
use crate::{SimBit, SimBv, SimValue};

//a Trait impls for bool
impl SimValue for bool {
    const BIT_WIDTH: usize = 1;
    const NYBBLE_WIDTH: usize = 1;
    const BYTE_WIDTH: usize = 1;
    const FMT_BIN: bool = true;
}

impl SimBit for bool {}

//a Macro for trait impl SimValue
macro_rules! impl_sim_value {
    ($t:ty, $nb:expr) => {
        impl SimValue for std::num::Wrapping<$t> {
            const BIT_WIDTH: usize = std::mem::size_of::<Self>() * 8;
            const NYBBLE_WIDTH: usize = std::mem::size_of::<Self>() * 2;
            const BYTE_WIDTH: usize = std::mem::size_of::<Self>();
            const FMT_HEX: bool = true;
            const FMT_BIN: bool = true;
        }
    };
}

//a Macro for trait impl SimBv
macro_rules! impl_sim_bv {
    ($t:ty, $nb:expr) => {
        impl SimBv for std::num::Wrapping<$t> {
            fn num_bits(&self) -> usize {
                $nb
            }
            fn as_u8s(&self) -> &[u8] {
                use std::num::Wrapping;
                unsafe {
                    std::slice::from_raw_parts(self as *const Wrapping<$t> as *const u8, $nb / 8)
                }
            }
            fn as_u8s_mut(&mut self) -> &mut [u8] {
                use std::num::Wrapping;
                unsafe {
                    std::slice::from_raw_parts_mut(self as *mut Wrapping<$t> as *mut u8, $nb / 8)
                }
            }
            fn try_as_u64s(&self) -> Option<&[u64]> {
                use std::num::Wrapping;
                Some(unsafe {
                    std::slice::from_raw_parts(self as *const Wrapping<$t> as *const u64, 1)
                })
            }

            fn try_as_u64s_mut(&mut self) -> Option<&mut [u64]> {
                if $nb < 64 {
                    return None;
                }
                use std::num::Wrapping;
                Some(unsafe {
                    std::slice::from_raw_parts_mut(self as *mut Wrapping<$t> as *mut u64, 1)
                })
            }

            fn try_as_u64(&self) -> Option<u64> {
                if $nb > 64 {
                    return None;
                }
                Some(self.0 as u64)
            }

            fn signed_neg(self) -> Self {
                use std::num::Wrapping;
                (!self) + Wrapping(1)
            }
        }
    };
}

//a Trait impls for std::num::Wrapping<u8, u16, u32, u64, u128>
impl_sim_value!(u8, 8);
impl_sim_bv!(u8, 8);

impl_sim_value!(u16, 16);
impl_sim_bv!(u16, 16);

impl_sim_value!(u32, 32);
impl_sim_bv!(u32, 32);

impl_sim_value!(u64, 64);
impl_sim_bv!(u64, 64);

impl_sim_value!(u128, 128);
impl_sim_bv!(u128, 128);
