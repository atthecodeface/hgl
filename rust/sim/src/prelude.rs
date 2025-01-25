// Move these into pub mod 'sim'

pub use crate::types::{Bit, Bv, BvN, IsBv};
pub use crate::{SimArray, SimBit, SimBv, SimStruct, SimValue};

pub mod sim {
    pub use crate::traits::{SimArray, SimBit, SimBv, SimStruct, SimValue, SimValueObject};
    pub use crate::types::{Bit, Bv, BvN, IsBv};
    pub use crate::types::{SimFormatValue, SimValueRef, SimValueRefMut};
    pub use crate::types::{SIM_FMT_AS_BIN, SIM_FMT_AS_HEX, SIM_FMT_HDR};
    pub use crate::{Component, Simulatable};
}

pub mod component {
    pub use super::sim::*;
    pub use crate::{ComponentBuilder, PortInfo, SimReset, Simulatable};
    pub use crate::{SimHandle, SimNsName, SimRegister};
}
