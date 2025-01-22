//a Imports
use crate::types::U8Ops;
use crate::utils;

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

//tt BvData
pub trait BvData: Sized + Copy + std::fmt::Debug + std::hash::Hash + std::default::Default {
    //mp zero
    fn zero<const NB: usize>(&mut self);

    //ap as_u8s (plain, mut, unbounded
    fn as_u8s_unbounded(&self) -> &[u8];
    fn as_u8s<const NB: usize>(&self) -> &[u8];
    fn as_u8s_mut<const NB: usize>(&mut self) -> &mut [u8];

    //mp cmp
    fn cmp<const NB: usize>(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        let nb = utils::num_u8_of_bits(NB);
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
        let s = self.as_u8s_mut::<NB>();
        let o = other.as_u8s::<NB>();
        for (i, m) in utils::iter_u8_of_bits(NB) {
            s[i] |= o[i] & m;
        }
    }

    //mp bit_and
    fn bit_and<const NB: usize>(&mut self, other: &Self) {
        let s = self.as_u8s_mut::<NB>();
        let o = other.as_u8s::<NB>();
        for (i, _m) in utils::iter_u8_of_bits(NB) {
            s[i] &= o[i];
        }
    }

    //mp bit_xor
    fn bit_xor<const NB: usize>(&mut self, other: &Self) {
        let s = self.as_u8s_mut::<NB>();
        let o = other.as_u8s::<NB>();
        for (i, m) in utils::iter_u8_of_bits(NB) {
            s[i] ^= o[i] & m;
        }
    }

    //mp bit_not
    fn bit_not<const NB: usize>(&mut self) {
        let s = self.as_u8s_mut::<NB>();
        for (i, m) in utils::iter_u8_of_bits(NB) {
            s[i] = (!s[i]) & m;
        }
    }

    //mp add_msk
    fn add_msk<const NB: usize>(&mut self, other: &Self) {
        let s = self.as_u8s_mut::<NB>();
        let o = other.as_u8s::<NB>();
        let mut c = 0;
        for (i, m) in utils::iter_u8_of_bits(NB) {
            let v = s[i] as u16 + o[i] as u16 + c;
            s[i] = (v as u8) & m;
            c = if v >= 256 { 1 } else { 0 };
        }
    }

    //mp sub_msk
    fn sub_msk<const NB: usize>(&mut self, other: &Self) {
        let s = self.as_u8s_mut::<NB>();
        let o = other.as_u8s::<NB>();
        let mut c = 1;
        for (i, m) in utils::iter_u8_of_bits(NB) {
            let v = s[i] as u16 + (!o[i]) as u16 + c;
            s[i] = (v as u8) & m;
            c = if v >= 256 { 1 } else { 0 };
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

    //zz All done
}
