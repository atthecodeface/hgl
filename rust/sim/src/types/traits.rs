//a Imports
use crate::types::U8Ops;
//a Traits
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

//fi mask_u8
fn mask_u8(n: usize) -> u8 {
    if n >= 8 {
        255
    } else {
        (1 << n) - 1
    }
}

//fi num_bytes
const fn num_bytes(n: usize) -> usize {
    (n + 7) / 8
}

//tt BvData
pub trait BvData: Sized + Copy + std::fmt::Debug + std::hash::Hash + std::default::Default {
    //mp zero
    fn zero<const NB: usize>(&mut self);

    //ap as_u8s (plain, mut, unbounded
    fn as_u8s_unbounded(&self) -> &[u8];
    fn as_u8s<const NB: usize>(&self) -> &[u8];
    fn as_u8s_mut<const NB: usize>(&mut self) -> &mut [u8];

    //mp set_u64
    fn set_u64<const NB: usize>(&mut self, mut value: u64) {
        let mut n = NB;
        for sd in self.as_u8s_mut::<NB>().iter_mut() {
            *sd = (value as u8) & mask_u8(n);
            value >>= 8;
            n -= 8;
        }
    }

    //mp cmp
    fn cmp<const NB: usize>(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        let nb = num_bytes(NB);
        let s = self.as_u8s::<NB>();
        let o = other.as_u8s::<NB>();
        for i in (0..nb).rev() {
            match s[i].cmp(&o[i]) {
                Equal => {
                    continue;
                }
                c => {
                    return c;
                }
            }
        }
        Equal
    }

    //mp bit_or
    fn bit_or<const NB: usize>(&mut self, other: &Self) {
        let mut n = NB;
        let s = self.as_u8s_mut::<NB>();
        let o = other.as_u8s::<NB>();
        for (sd, od) in s.iter_mut().zip(o.iter()) {
            *sd = (*sd | *od) & mask_u8(n);
            n -= 8;
        }
    }

    //mp bit_and
    fn bit_and<const NB: usize>(&mut self, other: &Self) {
        let mut n = NB;
        let s = self.as_u8s_mut::<NB>();
        let o = other.as_u8s::<NB>();
        for (sd, od) in s.iter_mut().zip(o.iter()) {
            *sd = (*sd & *od) & mask_u8(n);
            n -= 8;
        }
    }

    //mp bit_xor
    fn bit_xor<const NB: usize>(&mut self, other: &Self) {
        let mut n = NB;
        let s = self.as_u8s_mut::<NB>();
        let o = other.as_u8s::<NB>();
        for (sd, od) in s.iter_mut().zip(o.iter()) {
            *sd = (*sd ^ *od) & mask_u8(n);
            n -= 8;
        }
    }

    //mp bit_not
    fn bit_not<const NB: usize>(&mut self) {
        let mut n = NB;
        let s = self.as_u8s_mut::<NB>();
        for sd in s.iter_mut() {
            *sd = (!*sd) & mask_u8(n);
            n -= 8;
        }
    }

    //mp add_msk
    fn add_msk<const NB: usize>(&mut self, other: &Self) {
        let mut n = NB;
        let s = self.as_u8s_mut::<NB>();
        let o = other.as_u8s::<NB>();
        let mut c = 0;
        for (sd, od) in s.iter_mut().zip(o.iter()) {
            let v = (*sd) as u16 + (*od) as u16 + c;
            *sd = (v as u8) & mask_u8(n);
            c = if v >= 256 { 1 } else { 0 };
            n -= 8;
        }
    }

    //mp sub_msk
    fn sub_msk<const NB: usize>(&mut self, other: &Self) {
        let mut n = NB;
        let s = self.as_u8s_mut::<NB>();
        let o = other.as_u8s::<NB>();
        let mut c = 1;
        for (sd, od) in s.iter_mut().zip(o.iter()) {
            let v = (*sd) as u16 + (!*od) as u16 + c;
            *sd = (v as u8) & mask_u8(n);
            c = if v >= 256 { 1 } else { 0 };
            n -= 8;
        }
    }

    //mp bit_shl
    fn bit_shl<const NB: usize>(&mut self, by: usize) {
        let s = self.as_u8s_mut::<NB>();
        if by < NB {
            for i in 0..NB - by {
                let j = NB - 1 - i;
                let b = s.bit::<NB>(j - by);
                s.bit_set::<NB>(j, b);
            }
        }
        for i in 0..by {
            s.bit_set::<NB>(i, false);
        }
    }

    //mp bit_lshr
    fn bit_lshr<const NB: usize>(&mut self, by: usize) {
        let s = self.as_u8s_mut::<NB>();
        if by < NB {
            for i in 0..NB - by {
                let b = s.bit::<NB>(i + by);
                s.bit_set::<NB>(i, b);
            }
        }
        for i in 0..by {
            s.bit_set::<NB>(NB - 1 - i, false);
        }
    }

    //mp to_bin
    fn to_bin(&self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        let d = self.as_u8s_unbounded();
        for i in 0..n {
            let j = n - 1 - i;
            let bv = (d[j >> 8] >> (j & 7)) & 1;
            s.push((48 + bv) as char);
        }
        s
    }

    //mp to_hex
    fn to_hex(&self, n: usize) -> String {
        let nd = (n + 3) / 4;
        let mut s = String::with_capacity(nd);
        let d = self.as_u8s_unbounded();
        for i in 0..nd {
            let j = nd - 1 - i;
            let nv = if (j & 1) != 0 {
                d[j >> 1] >> 4
            } else {
                d[j >> 1] & 0xf
            };
            s.push(char::from_digit(nv as u32, 16).unwrap());
        }
        s
    }
}

//tt BvSim
// Add Serialize, Deserialize
//
// Make into SimBv
//
// Split into SimValue which is first set of traits supported by Bit and Struct
//
// Trait supported by struct?
//
// Trait supported by array of things?
pub trait BvSim:
    Sized
    + Copy
    + std::fmt::Debug
    + std::default::Default
    + std::cmp::PartialEq
    + std::cmp::Eq
    + std::cmp::PartialOrd
    + std::cmp::Ord
    + std::hash::Hash
    + std::convert::AsRef<[u8]>
    + std::convert::AsMut<[u8]>
    + std::ops::Not<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::ops::BitAnd<Self, Output = Self>
    + std::ops::BitAndAssign<Self>
    + std::ops::BitOr<Self, Output = Self>
    + std::ops::BitOrAssign<Self>
    + std::ops::BitXor<Self, Output = Self>
    + std::ops::BitXorAssign<Self>
    + std::ops::Add<Self, Output = Self>
    + std::ops::AddAssign<Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::SubAssign<Self>
    + std::ops::Shl<usize, Output = Self>
    + std::ops::ShlAssign<usize>
    + std::ops::Shr<usize, Output = Self>
    + std::ops::ShrAssign<usize>
{
    //ap num_bits - return size of the data in number of bits
    fn num_bits(&self) -> usize;

    //ap set_u64 - set to a u64 value, usually for testing
    fn set_u64(&mut self, mut value: u64) {
        let mut n = self.num_bits();
        let s = self.as_mut();
        for sd in s.iter_mut() {
            *sd = (value as u8) & mask_u8(n);
            value >>= 8;
            n -= 8;
        }
    }

    //ap try_as_u64s
    /// Return the data contents as a slice of u64, if possible given size and alignment
    fn try_as_u64s(&self) -> Option<&[u64]> {
        None
    }

    //ap try_as_u64s_mut
    /// Return the data contents as a slice of u64, if possible given size and alignment
    fn try_as_u64s_mut(&mut self) -> Option<&mut [u8]> {
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
            let s = self.as_ref();
            for sd in s.iter() {
                v += ((*sd) as u64) << n;
                n += 8;
            }
            Some(v)
        }
    }
}
