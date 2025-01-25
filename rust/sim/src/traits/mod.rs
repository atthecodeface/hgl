//a Modules
mod bv;
mod bv_data;
mod simulation;
mod types;

pub use bv::IsBv;
pub use bv_data::BvData;
pub use simulation::{Component, ComponentBuilder, Simulatable};
pub use simulation::{SimHandle, SimRegister};
pub use types::{SimArray, SimBit, SimBv, SimStruct, SimValue, SimValueObject};

//a Index
//tt Index
pub trait Index:
    Copy + std::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash + From<usize> + 'static
{
    fn index(self) -> usize;
}
pub trait Key: Copy + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static {}
impl Key for &'static str {}
