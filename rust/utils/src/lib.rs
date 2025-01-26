pub mod bit_ops;
pub mod fmt;
pub mod refs;
pub mod timer;
#[macro_use]
mod index;
mod indexed_slice;
mod indexed_vec;
mod strings_with_index;
mod vec_with_index;

pub mod index_vec {
    pub use super::index::Idx;
    pub use super::indexed_slice::IndexedSlice;
    pub use super::indexed_vec::IndexedVec;
    pub use super::strings_with_index::{StringIndex, StringsWithIndex};
    pub use super::vec_with_index::{IndexKey, VecWithIndex};
    pub use crate::make_index;
}
