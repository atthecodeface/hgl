//a Imports
use crate::data::{BitRange, BitRangeMut, U8Ops};
use crate::utils;
use crate::values::fmt;

//a Traits
//tt SimValueObject
/// Trait supported by SimBit, SimBv, etc
///
/// All values must provide this
///
/// This is an object-safe trait
pub trait SimValueObject: std::any::Any + std::fmt::Debug {
    fn as_any(&self) -> &dyn std::any::Any;
    fn bit_width(&self) -> usize {
        0
    }
    fn num_subelements(&self) -> usize {
        0
    }
    fn get_subelement(&self, _n: usize) -> Option<(&str, &dyn SimValueObject)> {
        None
    }

    //ap try_as_u8s
    /// Try to return the data contents as a slice of u8; this should
    /// return None if the underlying value is not Copy
    fn try_as_u8s(&self) -> Option<&[u8]> {
        None
    }

    //ap try_as_u8s_mut
    /// Try to return the data contents as a mutable slice of u8; this
    /// should return None if the underlying value is not Copy
    ///
    /// If a slice is returned and is to be updated, it will only be
    /// overwritten by a manual implementation of 'copy' from an
    /// identical type, so the value will remain valid.
    fn try_as_u8s_mut(&mut self) -> Option<&mut [u8]> {
        None
    }

    //mp might_equal
    /// Compare with what should be another SimValueObject
    ///
    /// Return true only if this type is Copy, other is the same type,
    /// and the bit-wise contents comparison of the data is equal
    fn might_equal(&self, _other: &dyn std::any::Any) -> bool {
        false
    }

    //mp fmt_with
    fn fmt_with(
        &self,
        _fmt: &mut std::fmt::Formatter,
        _style: usize,
    ) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

//it SimValueObject for T where SimValue
impl<T> SimValueObject for T
where
    T: SimValue,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn bit_width(&self) -> usize {
        <Self as SimValue>::BIT_WIDTH
    }
    fn num_subelements(&self) -> usize {
        <Self as SimValue>::NUM_SUBELEMENTS
    }
    fn get_subelement(&self, n: usize) -> Option<(&str, &dyn SimValueObject)> {
        <Self as SimValue>::get_subelement(self, n)
    }
    fn try_as_u8s(&self) -> Option<&[u8]> {
        Some(unsafe { utils::as_u8s(self) })
    }
    fn try_as_u8s_mut(&mut self) -> Option<&mut [u8]> {
        Some(unsafe { utils::as_u8s_mut(self) })
    }
    fn might_equal(&self, other: &dyn std::any::Any) -> bool {
        let Some(other) = other.downcast_ref::<Self>() else {
            return false;
        };
        let Some(od) = other.try_as_u8s() else {
            return false;
        };
        let sd = unsafe { utils::as_u8s(self) };
        sd == od
    }
    fn fmt_with(&self, fmt: &mut std::fmt::Formatter, style: usize) -> Result<(), std::fmt::Error> {
        let mut ascii_store = [b'0'; fmt::MAX_STRING_LENGTH];
        let mut ascii = ascii_store.as_mut_slice();
        let mut hdr_char = 'b';
        if (style & fmt::AS_HEX) != 0 && (<Self as SimValue>::FMT_HEX) {
            assert!(
                (<Self as SimValue>::BIT_WIDTH + 3) / 4 < fmt::MAX_STRING_LENGTH,
                "Need to restrict length of hex string"
            );
            hdr_char = 'h';
            ascii = &mut ascii[0..(<Self as SimValue>::BIT_WIDTH + 3) / 4];
            if !(<Self as SimValue>::fmt_hex(self, ascii)) {
                utils::fmt_hex(self, ascii);
            }
        } else if (style & fmt::AS_BIN) != 0 && (<Self as SimValue>::FMT_BIN) {
            assert!(
                <Self as SimValue>::BIT_WIDTH < fmt::MAX_STRING_LENGTH,
                "Need to restrict length of hex string"
            );
            ascii = &mut ascii[0..<Self as SimValue>::BIT_WIDTH];
            if !(<Self as SimValue>::fmt_bin(self, ascii)) {
                utils::fmt_bin(self, ascii);
            }
        }
        let ascii = unsafe { std::str::from_utf8_unchecked(ascii) };
        if (style & fmt::HDR) == 0 {
            fmt.write_str(ascii)
        } else {
            write!(
                fmt,
                "{}{}{}",
                <Self as SimValue>::BIT_WIDTH,
                hdr_char,
                ascii
            )
        }
    }
}

//tt SimValue
/// Trait supported by most simulatable values
///
/// This is *not* a dyn-compatible trait
pub trait SimValue:
    Sized
    + Copy
    + std::default::Default
    + std::cmp::PartialEq
    + std::cmp::Eq
    + std::hash::Hash
    + for<'de> serde::Deserialize<'de>
    + serde::Serialize
    + SimValueObject
{
    const BIT_WIDTH: usize;
    const NYBBLE_WIDTH: usize;
    const BYTE_WIDTH: usize;
    const FMT_HEX: bool = false;
    const FMT_BIN: bool = false;
    const NUM_SUBELEMENTS: usize = 0;

    fn get_subelement(&self, _n: usize) -> Option<(&str, &dyn SimValueObject)> {
        None
    }

    /// Implement this to override the default hex data-to-ascii
    /// conversion, which uses 'Self' as a slice of u8
    fn fmt_hex(&self, _ascii: &mut [u8]) -> bool {
        false
    }

    /// Implement this to override the default binary data-to-ascii
    /// conversion, which uses 'Self' as a slice of u8
    fn fmt_bin(&self, _ascii: &mut [u8]) -> bool {
        false
    }
}

//tt SimArray
pub trait SimArray<V: SimValue>:
    SimValue + std::ops::Index<usize, Output = V> + std::ops::IndexMut<usize>
{
    fn num_elements(&self) -> usize;
}

//tt SimStruct
pub trait SimStruct: SimValue {}

//tt SimBit
/// Any type that can be used as a single bit value by a simulation
pub trait SimBit
where
    bool: From<Self>,
    for<'a> &'a bool: From<&'a Self>,
    Self: SimValue
        + From<bool>
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
        + for<'a> std::ops::BitXorAssign<&'a Self>,
{
    fn randomize<F: FnMut() -> u64>(f: &mut F) -> Self {
        ((f() & 1) != 0).into()
    }

    #[inline]
    fn is_true(&self) -> bool {
        (*self).into()
    }

    #[inline]
    fn is_false(&self) -> bool {
        !self.is_true()
    }
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

    //ap of_u64 - create given a specific value
    fn of_u64(value: u64) -> Self {
        let mut s = Self::default();
        s.set_u64(value);
        s
    }

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
    ///
    /// This cannot fail
    fn as_u8s(&self) -> &[u8];

    //ap as_u8s
    /// Return the data contents as a mutable slice of u8
    ///
    /// This cannot fail
    fn as_u8s_mut(&mut self) -> &mut [u8];

    //mp signed_neg
    /// Treating the value as signed, perform a two's complement
    /// negation
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

    //mp bit
    /// Get a bit value
    #[track_caller]
    fn bit(&self, n: usize) -> bool {
        self.as_u8s().bit_nb_rt(n)
    }

    //mp bit_set
    /// Set a bit value
    #[track_caller]
    fn bit_set<I: Into<bool>>(&mut self, n: usize, v: I) {
        self.as_u8s_mut().bit_set_nb_rt(n, v.into())
    }

    //ap bit_range
    /// Return an immutable bit range (as a [BitRange]) using n bits
    /// starting at the specified lsb
    ///
    /// Panics if lsb+n is bigger than the vector size
    #[track_caller]
    fn bits(&self, lsb: usize, n: usize) -> BitRange<u8> {
        assert!(
            lsb + n <= self.num_bits(),
            "Bit selection outside the size of the bit vector"
        );
        BitRange::of_u8s(self.as_u8s(), lsb, n)
    }

    //ap bits_mut
    /// Return a mutable bit range (as a [BitRangeMut]) using n bits
    /// starting at the specified lsb
    ///
    /// Panics if lsb+n is bigger than the vector size
    fn bits_mut(&mut self, lsb: usize, n: usize) -> BitRangeMut<u8> {
        assert!(
            lsb + n <= self.num_bits(),
            "Bit selection outside the size of the bit vector"
        );
        BitRangeMut::of_u8s(self.as_u8s_mut(), lsb, n)
    }

    //fp is_zero
    /// Return true if the value is zero
    fn is_zero(&self) -> bool {
        !self.as_u8s().iter().any(|x| *x != 0)
    }

    //zz All done
}
