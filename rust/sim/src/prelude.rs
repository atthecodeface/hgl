// Move these into pub mod 'sim'

pub use crate::types::{Bit, Bv, BvN, IsBv};
pub use crate::{SimArray, SimBit, SimBv, SimStruct, SimValue};

pub mod sim {
    pub use crate::types::{Bit, Bv, BvN, IsBv};
    pub use crate::{Component, Simulatable};
    pub use crate::{SimArray, SimBit, SimBv, SimStruct, SimValue, SimValueObject};
}

pub mod component {
    pub use super::sim::*;
    pub use crate::{ComponentBuilder, PortData, PortDataMut, PortInfo, SimReset, Simulatable};
    pub use crate::{SimHandle, SimNsName, SimRegister};
}
