//a Modules
mod bv;
mod bv_data;
mod simulation;
mod types;

pub use bv::IsBv;
pub use bv_data::BvData;
pub use simulation::{Component, ComponentBuilder, Simulatable};
pub use simulation::{SimHandle, SimRegister};
pub use types::{SimArray, SimBit, SimBv, SimStruct};
pub use types::{SimCopyValue, SimValueAsU8s, SimValueObject};
