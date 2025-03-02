//a Imports
use serde::{Deserialize, Serialize};

use crate::data::{BitRange, BitRangeMut, U8Ops};
use crate::traits::{BvData, IsBv, SimBv, SimCopyValue, SimValueAsU8s};
use crate::value_types::Bit;

//a BvN
//tp BvN
/// This is a marker type, which uses specific trait implementations
/// to describe actual bit vector data
///
/// If this type (with NB bits) has an implementation of 'IsBv' then
/// *that* trait implementation describes the backing store data type
/// to be used for such a vector
pub struct BvN<const NB: usize>();

//a Bv
//tp Bv
/// A bit vector type of a given number of bits
///
/// This requires there to be an implementation of IsBv for the marker
/// type BvN<NB>; that marker type provides the backing store used
/// here, and other vector-specific data and methods.
///
/// Index and IndexMut are not supported as there is nothing to return
/// a reference to for part of a [Bv]
///
/// And, Or, Xor, Not, Add and Sub (with assign) are supported with the rhs being Self or a reference to Self
#[repr(transparent)]
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Bv<const NB: usize>
where
    BvN<{ NB }>: IsBv,
{
    data: <BvN<{ NB }> as IsBv>::BackingStore,
}

//ip Bv
impl<const NB: usize> Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    //cp of_bit_range
    pub fn of_bit_range<'a, I: Into<BitRange<'a, u8>>>(br: I) -> Self {
        let mut s = Self::default();
        s.bit_range_mut(0, NB).set::<NB>(br.into());
        s
    }

    //fi as_u8s
    /// Return a reference to the data as a u8 slice
    pub fn as_u8s(&self) -> &[u8] {
        self.data.as_u8s::<NB>()
    }

    //fi as_u8s_mut
    /// Return a reference to the data as a u8 slice
    pub fn as_u8s_mut(&mut self) -> &mut [u8] {
        self.data.as_u8s_mut::<NB>()
    }

    //mp zero
    /// Clear the bit vector
    pub fn zero(&mut self) {
        self.data.zero::<NB>();
    }

    //mp bit_as_bit
    /// Get a bit value
    #[track_caller]
    pub fn bit_as_bit(&self, n: usize) -> Bit {
        self.as_u8s().bit::<NB>(n).into()
    }

    //mp bit
    /// Get a bit value
    #[track_caller]
    pub fn bit(&self, n: usize) -> bool {
        self.as_u8s().bit::<NB>(n)
    }

    //mp bit_set
    /// Set a bit value
    #[track_caller]
    pub fn bit_set<I: Into<bool>>(&mut self, n: usize, v: I) {
        self.as_u8s_mut().bit_set::<NB>(n, v.into())
    }

    //ap bit_range_to_bv
    pub fn bit_range_to_bv<const BVN: usize>(&self, lsb: usize, n: usize) -> Bv<BVN>
    where
        BvN<BVN>: IsBv,
    {
        Bv::<BVN>::of_bit_range(self.bit_range(lsb, n))
    }

    //ap as_bit_range
    pub fn as_bit_range(&self) -> BitRange<u8> {
        BitRange::of_u8s(self.as_u8s(), 0, NB)
    }

    //ap bit_range
    pub fn bit_range(&self, lsb: usize, n: usize) -> BitRange<u8> {
        BitRange::of_u8s(self.as_u8s(), lsb, n)
    }

    //ap bit_range_mut
    pub fn bit_range_mut(&mut self, lsb: usize, n: usize) -> BitRangeMut<u8> {
        BitRangeMut::of_u8s(self.as_u8s_mut(), lsb, n)
    }

    //fp is_zero
    /// Return true if the value is zero
    pub fn is_zero(&self) -> bool {
        !self.as_u8s().iter().any(|x| *x != 0)
    }
}

//ip Debug for Bv
impl<const NB: usize> std::fmt::Debug for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if NB < 8 {
            write!(fmt, "{}b{}", NB, self.data.to_bin(NB))
        } else {
            write!(fmt, "{}h{}", NB, self.data.to_hex(NB))
        }
    }
}

//ip AsRef[u8] for Bv - do we want this?
impl<const NB: usize> std::convert::AsRef<[u8]> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn as_ref(&self) -> &[u8] {
        self.as_u8s()
    }
}

//ip AsRefMut[u8] for Bv - do we want this?
impl<const NB: usize> std::convert::AsMut<[u8]> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_u8s_mut()
    }
}

//ip PartialEq/Eq implementations
impl<const NB: usize> std::cmp::PartialEq for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl<const NB: usize> std::cmp::Eq for Bv<NB> where BvN<{ NB }>: IsBv {}

//ip PartialOrd/Ord implementations
impl<const NB: usize> std::cmp::PartialOrd for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const NB: usize> std::cmp::Ord for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.data.cmp::<NB>(&other.data)
    }
}

//ip Hash implementation
impl<const NB: usize> std::hash::Hash for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn hash<H>(&self, h: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.data.hash(h)
    }
}

//ip Not implementations - note Neg would require Bv to be signed, and it is not
impl<const NB: usize> std::ops::Not for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    type Output = Self;
    fn not(self) -> Self {
        let mut s = self;
        s.data.bit_not::<NB>();
        s
    }
}

//ip BitAnd/BitOr/BitXor/Add/Sub implementations
macro_rules! bit_op {
    ($ts:ty, $tr:ty, $tsa:ty, $tra:ty, $fc:ident, $fa:ident, $op:ident) => {
        impl<const NB: usize> $ts for Bv<NB>
        where
            BvN<{ NB }>: IsBv,
        {
            type Output = Self;
            fn $fc(self, other: Bv<NB>) -> Self {
                let mut s = self;
                s.data.$op::<NB>(&other.data);
                s
            }
        }
        impl<const NB: usize> $tr for Bv<NB>
        where
            BvN<{ NB }>: IsBv,
        {
            type Output = Self;
            fn $fc(self, other: &Bv<NB>) -> Self {
                let mut s = self;
                s.data.$op::<NB>(&other.data);
                s
            }
        }
        impl<const NB: usize> $tsa for Bv<NB>
        where
            BvN<{ NB }>: IsBv,
        {
            fn $fa(&mut self, other: Bv<NB>) {
                self.data.$op::<NB>(&other.data);
            }
        }
        impl<const NB: usize> $tra for Bv<NB>
        where
            BvN<{ NB }>: IsBv,
        {
            fn $fa(&mut self, other: &Bv<NB>) {
                self.data.$op::<NB>(&other.data);
            }
        }
    };
}
bit_op!(
    std::ops::BitOr<Self>,
    std::ops::BitOr<&Self>,
    std::ops::BitOrAssign<Self>,
    std::ops::BitOrAssign<&Self>,
    bitor,
    bitor_assign,
    bit_or
);
bit_op!(
    std::ops::BitAnd<Self>,
    std::ops::BitAnd<&Self>,
    std::ops::BitAndAssign<Self>,
    std::ops::BitAndAssign<&Self>,
    bitand,
    bitand_assign,
    bit_and
);
bit_op!(
    std::ops::BitXor<Self>,
    std::ops::BitXor<&Self>,
    std::ops::BitXorAssign<Self>,
    std::ops::BitXorAssign<&Self>,
    bitxor,
    bitxor_assign,
    bit_xor
);
bit_op!(
    std::ops::Add<Self>,
    std::ops::Add<&Self>,
    std::ops::AddAssign<Self>,
    std::ops::AddAssign<&Self>,
    add,
    add_assign,
    add_msk
);
bit_op!(
    std::ops::Sub<Self>,
    std::ops::Sub<&Self>,
    std::ops::SubAssign<Self>,
    std::ops::SubAssign<&Self>,
    sub,
    sub_assign,
    sub_msk
);

//ip Shl/Shr implementations
impl<const NB: usize> std::ops::Shl<usize> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        let mut s = self;
        s.data.bit_shl::<NB>(rhs);
        s
    }
}

impl<const NB: usize> std::ops::ShlAssign<usize> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn shl_assign(&mut self, rhs: usize) {
        self.data.bit_shl::<NB>(rhs);
    }
}

impl<const NB: usize> std::ops::Shr<usize> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        let mut s = self;
        s.data.bit_lshr::<NB>(rhs);
        s
    }
}

impl<const NB: usize> std::ops::ShrAssign<usize> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn shr_assign(&mut self, rhs: usize) {
        self.data.bit_lshr::<NB>(rhs);
    }
}

//ip SimCopyValue for Bv
impl<const NB: usize> SimCopyValue for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    const BIT_WIDTH: usize = NB;
    const NYBBLE_WIDTH: usize = (NB + 3) / 4;
    const BYTE_WIDTH: usize = (NB + 7) / 8;
    const FMT_HEX: bool = true;
    const FMT_BIN: bool = true;
}

//ip SimValueAsU8s for Bv
impl<const NB: usize> SimValueAsU8s for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn as_u8s(&self) -> &[u8] {
        self.data.as_u8s::<NB>()
    }
    fn as_u8s_mut(&mut self) -> &mut [u8] {
        self.data.as_u8s_mut::<NB>()
    }
}

//ip SimBv for Bv
impl<const NB: usize> SimBv for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    #[inline]
    fn num_bits(&self) -> usize {
        NB
    }
    fn signed_neg(self) -> Self {
        let mut s = Self::default();
        s.data.sub_msk::<NB>(&self.data);
        s
    }
}

//ip From<u64> for Bv
impl<const NB: usize> From<u64> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn from(v: u64) -> Self {
        <Bv<{ NB }> as SimBv>::of_u64(v)
    }
}
