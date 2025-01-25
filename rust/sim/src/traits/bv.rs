//a Imports
use crate::traits::BvData;

//a IsBv trait
//tt IsBv
/// Trait that describes the storage for a bit vector of NB bits with
/// a specific backing store
pub trait IsBv {
    /// The storage type used
    type BackingStore: BvData;

    /// Number of bits in the vector
    const NB: usize;

    /// Number of u8 that are *valid* in the backing store
    ///
    /// BvData :: as_u8s will return an &[u8] that may be longer than
    /// is *valid* for the data; this value should be no more than the
    /// length of that [u8], and may well be less (for example, 7 for
    /// a 56-bit vector)
    const NU8: usize;
}
