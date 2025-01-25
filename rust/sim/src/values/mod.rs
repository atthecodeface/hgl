mod sim_format_value;
mod sim_value_ref;

pub mod fmt {
    pub const MAX_STRING_LENGTH: usize = 256;

    pub const AS_HEX: usize = 1;
    pub const AS_BIN: usize = 2;
    pub const HDR: usize = 4;
    pub const FULL: usize = AS_BIN | HDR | AS_HEX;
}

pub use sim_format_value::SimFormatValue;
pub use sim_value_ref::{SimValueRef, SimValueRefMut};
