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

//ip Debug for BitSet
impl<I: Idx> std::fmt::Debug for BitSet<I> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let n = self.num_bits();
        write!(fmt, "{n}:")?;
        for (i, b) in self.iter_bits().enumerate() {
            if b {
                write!(fmt, "1")?;
            } else {
                write!(fmt, "0")?;
            }
            if ((n - i) % 8) == 0 {
                write!(fmt, "_")?;
            }
        }
        Ok(())
    }
}

//tp BitSetIter
pub struct BitSetIter<'a> {
    i: usize,
    n: usize,
    dw: &'a [u64],
}
impl<'a> std::iter::Iterator for BitSetIter<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.i >= self.n {
            None
        } else {
            let bi = self.i & 63;
            let b = 1 << bi;
            let wi = self.i / 64;
            self.i += 1;
            Some((self.dw[wi] & b) != 0)
        }
    }
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

    //mi assert_size_match
    #[track_caller]
    fn assert_size_match(&self, other: &Self) {
        let s = self.num_bits();
        let o = other.num_bits();
        assert_eq!(s, o, "Mismatch in BitSet sizes ({s}, {o})");
    }

    //mi data
    fn data(&self) -> &[u64] {
        match self {
            Self::Bits { data, .. } => data,
            Self::Array { data, .. } => &*data,
        }
    }

    //mi iter_data_mut
    fn iter_data_mut(&mut self) -> impl Iterator<Item = &mut u64> {
        match self {
            Self::Bits { data, .. } => data.iter_mut(),
            Self::Array { data, .. } => data.iter_mut(),
        }
    }

    //mi iter_bits
    fn iter_bits(&self) -> BitSetIter {
        let n = self.num_bits();
        let dw = self.data();
        BitSetIter { i: 0, n, dw }
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
        for (s, o) in self.iter_data_mut().zip(other.data().iter()) {
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
        for (s, o) in self.iter_data_mut().zip(other.data().iter()) {
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
//tf test_bitset_size
/// Check that the BitSet is 32 bits on 64-bit processor architectures
#[test]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
fn test_bitset_size() {
    assert_eq!(std::mem::size_of::<BitSet<usize>>(), 32);
}

//tf test_bitset
/// Check that bits can be set and cleared individually
#[test]
fn test_bitset() {
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

//tf test_bitset
/// Check that bits can be set and cleared individually
#[test]
fn test_bitset_and_or() {
    let mut bit_pairs = vec![];
    let mut bits_to_set_a = vec![];
    let mut bits_to_set_b = vec![];
    let mut seed: std::num::Wrapping<u32> = std::num::Wrapping(0x12345678);
    for i in 0..800 {
        seed = seed * std::num::Wrapping(1293) + std::num::Wrapping(3) + (seed >> 16);
        let a = seed & std::num::Wrapping(1 << 30) != std::num::Wrapping(0);
        let b = seed & std::num::Wrapping(1 << 14) != std::num::Wrapping(0);
        if a {
            bits_to_set_a.push(i);
        }
        if b {
            bits_to_set_b.push(i);
        }
        bit_pairs.push((a, b));
    }
    for i in 0..10 {
        for j in 62..65 {
            let n = 64 * i + j;
            let mut x: BitSet<usize> = BitSet::none(n);
            let mut y: BitSet<usize> = BitSet::none(n);

            for a in &bits_to_set_a {
                if *a >= n {
                    break;
                }
                x.set(*a, true);
            }
            for b in &bits_to_set_b {
                if *b >= n {
                    break;
                }
                y.set(*b, true);
            }
            // eprintln!("{x:?}, {y:?}");
            let x_and_y = &x & &y;
            let x_or_y = &x | &y;
            let mut not_x_nor_y = x_or_y.clone();
            not_x_nor_y.complement();
            for (i, (a, b)) in bit_pairs.iter().enumerate() {
                if i >= n {
                    break;
                }
                assert_eq!(*a, x.is_set(i));
                assert_eq!(*b, y.is_set(i));
                assert_eq!(*a & *b, x_and_y.is_set(i));
                assert_eq!(*a | *b, x_or_y.is_set(i));
                assert_eq!(!*a & !*b, not_x_nor_y.is_set(i));
            }
        }
    }
}
