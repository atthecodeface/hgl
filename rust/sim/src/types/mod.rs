mod bit;
mod bit_range;
mod bv;
mod bv_data;
mod bv_sim64;
mod bv_sim64_array;
mod sim_format_value;
mod sim_value_ref;
mod std;
mod u8_ops;

pub mod fmt {
    pub const MAX_STRING_LENGTH: usize = 256;

    pub const AS_HEX: usize = 1;
    pub const AS_BIN: usize = 2;
    pub const HDR: usize = 4;
    pub const FULL: usize = AS_BIN | HDR | AS_HEX;
}

pub use bit::Bit;
pub use bit_range::{BitRange, BitRangeMut};
pub use bv::{Bv, BvN, IsBv};
pub use bv_data::BvData;
pub use sim_format_value::SimFormatValue;
pub use sim_value_ref::{SimValueRef, SimValueRefMut};
pub(crate) use u8_ops::U8Ops;
