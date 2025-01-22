use crate::utils;

//a Traits
//tt SimValue
/// Trait supported by SimBit, SimBv, etc
///
/// All values must provide this
///
/// Add Serialize, Deserialize
pub trait SimValue:
    Sized
    + Copy
    + std::fmt::Debug
    + std::default::Default
    + std::cmp::PartialEq
    + std::cmp::Eq
    + std::hash::Hash
    + std::any::Any
{
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait SimArray<V: SimValue>:
    SimValue + std::ops::Index<usize, Output = V> + std::ops::IndexMut<usize>
{
    fn num_elements(&self) -> usize;
}

pub trait SimStruct: SimValue {}

//tt SimBit
///
pub trait SimBit:
    SimValue
    + std::cmp::PartialOrd
    + std::cmp::Ord
    + std::ops::Not<Output = Self>
    + std::ops::BitAnd<Self, Output = Self>
    + std::ops::BitAndAssign<Self>
    + for<'a> std::ops::BitAnd<&'a Self, Output = Self>
    + for<'a> std::ops::BitAndAssign<&'a Self>
    + std::ops::BitOr<Self, Output = Self>
    + std::ops::BitOrAssign<Self>
    + for<'a> std::ops::BitOr<&'a Self, Output = Self>
    + for<'a> std::ops::BitOrAssign<&'a Self>
    + std::ops::BitXor<Self, Output = Self>
    + std::ops::BitXorAssign<Self>
    + for<'a> std::ops::BitXor<&'a Self, Output = Self>
    + for<'a> std::ops::BitXorAssign<&'a Self>
{
    fn randomize<F: FnMut() -> u64>(f: &F) -> Self;
}

//tt SimBv
/// A trait required to be supported by types that can be used as
/// bit-vectors by the simulation
pub trait SimBv:
    SimValue
    + std::cmp::PartialOrd
    + std::cmp::Ord
    + std::ops::Not<Output = Self>
    + std::ops::BitAnd<Self, Output = Self>
    + std::ops::BitAndAssign<Self>
    + for<'a> std::ops::BitAnd<&'a Self, Output = Self>
    + for<'a> std::ops::BitAndAssign<&'a Self>
    + std::ops::BitOr<Self, Output = Self>
    + std::ops::BitOrAssign<Self>
    + for<'a> std::ops::BitOr<&'a Self, Output = Self>
    + for<'a> std::ops::BitOrAssign<&'a Self>
    + std::ops::BitXor<Self, Output = Self>
    + std::ops::BitXorAssign<Self>
    + for<'a> std::ops::BitXor<&'a Self, Output = Self>
    + for<'a> std::ops::BitXorAssign<&'a Self>
    + std::ops::Add<Self, Output = Self>
    + std::ops::AddAssign<Self>
    + for<'a> std::ops::Add<&'a Self, Output = Self>
    + for<'a> std::ops::AddAssign<&'a Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::SubAssign<Self>
    + for<'a> std::ops::Sub<&'a Self, Output = Self>
    + for<'a> std::ops::SubAssign<&'a Self>
    + std::ops::Shl<usize, Output = Self>
    + std::ops::ShlAssign<usize>
    + std::ops::Shr<usize, Output = Self>
    + std::ops::ShrAssign<usize>
{
    //cp randomize
    /// Create a random value, given a function that returns random numbers
    ///
    /// This must return consistent values given the same value of f
    fn randomize<F: FnMut() -> u64>(f: &mut F) -> Self {
        let mut s = Self::default();
        let n = s.num_bits();
        if let Some(sd) = s.try_as_u64s_mut() {
            for (i, m) in utils::iter_u64_of_bits(n) {
                sd[i] = f() & m;
            }
        } else {
            let sd = s.as_u8s_mut();
            for (i, m) in utils::iter_u8_of_bits(n) {
                sd[i] = (f() as u8) & m;
            }
        }
        s
    }

    //ap num_bits - return size of the data in number of bits
    fn num_bits(&self) -> usize;

    //ap set_u64 - set to a u64 value, usually for testing
    fn set_u64(&mut self, mut value: u64) {
        let n = self.num_bits();
        let sd = self.as_u8s_mut();
        for (i, m) in utils::iter_u8_of_bits(n) {
            sd[i] = (value as u8) & m;
            value >>= 8;
        }
    }

    //ap as_u8s
    /// Return the data contents as a slice of u8
    fn as_u8s(&self) -> &[u8];
    fn as_u8s_mut(&mut self) -> &mut [u8];

    fn signed_neg(self) -> Self;
    //ap try_as_u64s
    /// Return the data contents as a slice of u64, if possible given size and alignment
    fn try_as_u64s(&self) -> Option<&[u64]> {
        None
    }

    //ap try_as_u64s_mut
    /// Return the data contents as a slice of u64, if possible given size and alignment
    fn try_as_u64s_mut(&mut self) -> Option<&mut [u64]> {
        None
    }

    //ap try_as_u64
    fn try_as_u64(&self) -> Option<u64> {
        if self.num_bits() > 64 {
            None
        } else if let Some(v) = self.try_as_u64s() {
            Some(v[0])
        } else {
            let mut v = 0;
            let mut n = 0;
            let s = self.as_u8s();
            for sd in s.iter() {
                v += ((*sd) as u64) << n;
                n += 8;
            }
            Some(v)
        }
    }

    //zz All done
}
