use std::marker::PhantomData;

use hgl_indexed_vec::Idx;

//a BitSet
//tp BitSet
/// A set of bits of any size
///
/// This type always fits within 32 bytes; it encodes a set of 192 bits in that size, but for more it uses a Vec
#[derive(Clone, PartialEq, Eq)]
pub enum BitSet<I: Idx> {
    Array {
        n: u32,
        data: Vec<u64>,
    },
    Bits {
        n: u8,
        nw: u8,
        nb: u8,
        data: [u64; 3],
        _phantom: PhantomData<I>,
    },
}

//ip BitSet
impl<I: Idx> BitSet<I> {
    //fi word_index
    /// Return the index of bit N
    #[inline]
    const fn word_index(n: usize) -> usize {
        n / 64
    }

    //fi bit_index
    /// Return the bit of bit N
    #[inline]
    const fn bit_index(n: usize) -> usize {
        n & 63
    }

    //fi nw
    /// Return number of 64-bit words required
    #[inline]
    const fn nw(n: usize) -> usize {
        (n + 63) / 64
    }

    //fi nb
    /// Return number of bits in the last word (1-64)
    #[inline]
    const fn nb(n: usize) -> usize {
        match n & 63 {
            0 => 64,
            n => n,
        }
    }

    //fi mask
    /// Return number of 64-bit words required
    #[inline]
    const fn mask(n: usize) -> u64 {
        match n & 63 {
            0 => !0,
            n => (1 << n) - 1,
        }
    }

    //cp none
    /// Create an empty BitSet
    #[must_use]
    pub fn none(n: usize) -> Self {
        let nw = Self::nw(n);
        let nb = Self::nb(n);
        if nw > 3 {
            Self::Array {
                n: n as u32,
                data: vec![0; nw],
            }
        } else {
            Self::Bits {
                n: n as u8,
                nw: nw as u8,
                nb: nb as u8,
                data: [0_u64; 3],
                _phantom: PhantomData,
            }
        }
    }

    //cp all
    /// Create a new BitSet with all the bits set
    #[must_use]
    pub fn all(n: usize) -> Self {
        let mut s = Self::none(n);
        s.complement();
        s
    }

    //ap num_bits
    /// Get the number of bits in the BitSet
    pub fn num_bits(&self) -> usize {
        match self {
            Self::Bits { n, .. } => *n as usize,
            Self::Array { n, .. } => *n as usize,
        }
    }

    //ap is_empty
    /// Return true if the BitSet is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Bits { data, .. } => data == &[0; 3],
            Self::Array { data, .. } => data.iter().all(|s| *s == 0),
        }
    }

    //mi data_word
    /// Get a reference to the data word containing bit n
    fn data_word(&self, index: I) -> Option<(&u64, usize)> {
        let index = index.index();
        let wi = Self::word_index(index);
        let bi = Self::bit_index(index);
        match self {
            Self::Bits { n, ref data, .. } => {
                if index >= *n as usize {
                    None
                } else {
                    Some((&data[wi], bi))
                }
            }
            Self::Array { n, ref data } => {
                if index >= *n as usize {
                    None
                } else {
                    Some((&data[wi], bi))
                }
            }
        }
    }

    //mi data_word_mut
    /// Get a mutable reference to the data word containing bit n
    fn data_word_mut(&mut self, index: I) -> Option<(&mut u64, usize)> {
        let index = index.index();
        let wi = Self::word_index(index);
        let bi = Self::bit_index(index);
        match self {
            Self::Bits {
                n, ref mut data, ..
            } => {
                if index >= *n as usize {
                    None
                } else {
                    Some((&mut data[wi], bi))
                }
            }
            Self::Array { n, ref mut data } => {
                if index >= *n as usize {
                    None
                } else {
                    Some((&mut data[wi], bi))
                }
            }
        }
    }

    //ap is_set
    /// Return true if an index is set
    #[track_caller]
    pub fn is_set(&self, n: I) -> bool {
        let (data, bit) = self.data_word(n).expect("Index {n} out of range");
        ((*data >> bit) & 1) != 0
    }

    //ap is_unset
    /// Return true if an index is not set
    #[track_caller]
    pub fn is_unset(&self, n: I) -> bool {
        let (data, bit) = self.data_word(n).expect("Index {n} out of range");
        ((*data >> bit) & 1) == 0
    }

    //ap set
    /// Return true if an index is set
    #[track_caller]
    pub fn set(&mut self, n: I, value: bool) {
        let (data, bit) = self.data_word_mut(n).expect("Index {n} out of range");
        let b = 1 << bit;
        if value {
            *data |= b;
        } else {
            *data &= !b;
        }
    }

    //mi iter_data
    fn iter_data(&self) -> impl Iterator<Item = &u64> {
        match self {
            Self::Bits { data, .. } => data.iter(),
            Self::Array { data, .. } => data.iter(),
        }
    }

    //mi assert_size_match
    #[track_caller]
    fn assert_size_match(&self, other: &Self) {
        let s = self.num_bits();
        let o = other.num_bits();
        assert_eq!(s, o, "Mismatch in BitSet sizes ({s}, {o})");
    }

    //mi iter_data_mut
    fn iter_data_mut(&mut self) -> impl Iterator<Item = &mut u64> {
        match self {
            Self::Bits { data, .. } => data.iter_mut(),
            Self::Array { data, .. } => data.iter_mut(),
        }
    }

    //mp complement
    /// Complement the vector
    pub fn complement(&mut self) {
        match self {
            Self::Bits {
                nw,
                nb,
                ref mut data,
                ..
            } => {
                let m = Self::mask(*nb as usize);
                match *nw {
                    1 => {
                        data[0] ^= m;
                    }
                    2 => {
                        data[0] = !data[0];
                        data[1] ^= m;
                    }
                    _ => {
                        data[0] = !data[0];
                        data[1] = !data[1];
                        data[2] ^= m;
                    }
                }
            }
            Self::Array { n, ref mut data } => {
                let nw = Self::nw(*n as usize);
                let mask = Self::mask(*n as usize);
                for (i, d) in data.iter_mut().enumerate() {
                    if i + 1 == nw {
                        *d ^= mask;
                    } else {
                        *d = !*d;
                    }
                }
            }
        }
    }
}

//ip BitAndAssign for BitSet
impl<I: Idx> std::ops::BitAndAssign<&BitSet<I>> for BitSet<I> {
    #[track_caller]
    fn bitand_assign(&mut self, other: &Self) {
        self.assert_size_match(other);
        for (s, o) in self.iter_data_mut().zip(other.iter_data()) {
            *s &= *o;
        }
    }
}

//ip BitAndAssign for BitSet
impl<'a, I: Idx> std::ops::BitAnd<&'a BitSet<I>> for &'a BitSet<I> {
    type Output = BitSet<I>;
    #[track_caller]
    fn bitand(self, other: &BitSet<I>) -> BitSet<I> {
        let mut result = self.clone();
        result &= other;
        result
    }
}

//ip BitOrAssign for BitSet
impl<I: Idx> std::ops::BitOrAssign<&BitSet<I>> for BitSet<I> {
    #[track_caller]
    fn bitor_assign(&mut self, other: &Self) {
        self.assert_size_match(other);
        for (s, o) in self.iter_data_mut().zip(other.iter_data()) {
            *s |= *o;
        }
    }
}

//ip BitOrAssign for BitSet
impl<'a, I: Idx> std::ops::BitOr<&'a BitSet<I>> for &'a BitSet<I> {
    type Output = BitSet<I>;
    #[track_caller]
    fn bitor(self, other: &BitSet<I>) -> BitSet<I> {
        let mut result = self.clone();
        result |= other;
        result
    }
}

//a Tests
#[test]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
fn test_bitset() {
    assert_eq!(std::mem::size_of::<BitSet<usize>>(), 32);

    for i in 0..10 {
        for j in 62..65 {
            let n = 64 * i + j;
            let mut x: BitSet<usize> = BitSet::none(n);
            eprintln!("Test BitSet of size {n}");
            for k in 0..n {
                assert!(x.is_empty(), "BitSet must be empty");
                assert!(!x.is_set(k), "Bit {k} must not be set");
                assert!(x.is_unset(k), "Bit {k} must be unset");
                x.set(k, true);
                assert!(!x.is_empty(), "BitSet must be nonempty");
                assert!(x.is_set(k), "Bit {k} must not be set");
                assert!(!x.is_unset(k), "Bit {k} must be unset");
                x.complement();
                assert!(!x.is_set(k), "Bit {k} must not be set");
                assert!(x.is_unset(k), "Bit {k} must be unset");
                x.complement();
                assert!(x.is_set(k), "Bit {k} must not be set");
                assert!(!x.is_unset(k), "Bit {k} must be unset");
                x.set(k, false);
                assert!(!x.is_set(k), "Bit {k} must not be set");
                assert!(x.is_unset(k), "Bit {k} must be unset");
            }
        }
    }
}
