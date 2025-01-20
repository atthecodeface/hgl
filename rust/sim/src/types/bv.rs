//a Imports
use crate::types::{U8Ops, BvData, IsBv, BitRange, BitRangeMut};

//a BvN
//tp BvN
/// This is a marker type, which uses specific trait implementations
/// to describe actual bit vector data
///
/// If this type (with NB bits) has an implementation of 'IsBv' then
/// *that* trait implementation describes the backing store data type
/// to be used for such a vector
pub struct BvN<const NB: usize> ();


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
#[derive(Clone, Copy, Default)]
pub struct Bv<const NB: usize>
where
    BvN<{ NB }>: IsBv,
{
    data: <BvN<{ NB }> as IsBv>::BackingStore,
}

//ip Bv
impl <const NB: usize> Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    //fi as_u8s
    /// Return a reference to the data as a u8 slice
    pub fn as_u8s(&self) -> &[u8] {
        self.data.as_u8s(<BvN<{ NB }> as IsBv>::NU8)
    }

    //fi as_u8s
    /// Return a reference to the data as a u8 slice
    pub fn as_u8s_mut(&mut self) -> &mut [u8] {
        self.data.as_u8s_mut(<BvN<{ NB }> as IsBv>::NU8)
    }

    //mp zero
    /// Clear the bit vector
    pub fn zero(&mut self) {
        self.data.zero();
    }

    //mp bit
    /// Get a bit value
    #[track_caller]
    pub fn bit(&self, n:usize) -> bool {
        self.as_u8s().bit::<NB>(n)
    }

    //mp bit_set
    /// Set a bit value
    #[track_caller]
    pub fn bit_set<I: Into<bool>>(&mut self, n:usize, v:I) {
        self.as_u8s_mut().bit_set::<NB>(n, v.into())
    }

    //ap bit_range
    pub fn bit_range(&self, lsb:usize, n:usize) -> BitRange<u8> {
        BitRange::of_u8s(self.as_u8s(), lsb, n)
    }
  
   //ap bit_range_mut
    pub fn bit_range_mut(&mut self, lsb:usize, n:usize) -> BitRangeMut<u8> {
        BitRangeMut::of_u8s(self.as_u8s_mut(), lsb, n)
    }

    //fp is_zero
    /// Return true if the value is zero
    pub fn is_zero(&self) -> bool {
        self.as_u8s().iter().position(|x| *x!=0).is_none()
    }
}

//ip Debug for Bv
impl <const NB: usize> std::fmt::Debug for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if NB < 8 {
            write!(fmt, "{}b{}", NB, self.to_bin(NB))
        } else {
            write!(fmt, "{}h{}", NB, self.to_hex(NB))
        }
    }
}

//ip Deref for Bv - do we want this?
impl <const NB: usize> std::ops::Deref for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    type Target = <BvN<{ NB }> as IsBv>::BackingStore;
    fn deref(&self) -> &<BvN<{ NB }> as IsBv>::BackingStore {
        &self.data
    }
}

//ip DerefMut for Bv - do we want this?
impl <const NB: usize> std::ops::DerefMut for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn deref_mut(&mut self) -> &mut <BvN<{ NB }> as IsBv>::BackingStore {
        &mut self.data
    }
}

//ip AsRef[u8] for Bv - do we want this?
impl <const NB: usize> std::convert::AsRef<[u8]> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn as_ref(&self) -> &[u8] {
        self.as_u8s()
    }
}

//ip AsRefMut[u8] for Bv - do we want this?
impl <const NB: usize> std::convert::AsMut<[u8]> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_u8s_mut()
    }
}

//ip Not/Neg implementations
impl <const NB: usize> std::ops::Not for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    type Output = Self;
    fn not(self) -> Self {
        let mut s = self;
        s.bit_not::<NB>();
        s
    }
}

impl <const NB: usize> std::ops::Neg for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    type Output = Self;
    fn neg(self) -> Self {
        let mut s = Self::default();
        s.sub_msk(&self, NB);
        s
    }
}

//ip BitAnd/BitOr/BitXor/Add/Sub implementations
macro_rules! bit_op {
    ($ts:ty, $tr:ty, $tsa:ty, $tra:ty, $fc:ident, $fa:ident, $op:ident) => {
impl <const NB: usize> $ts for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    type Output = Self;
    fn $fc(self, other: Bv<NB>) -> Self {
        let mut s = self;
        s.$op(&other, NB);
        s
    }
}
impl <const NB: usize> $tr for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    type Output = Self;
    fn $fc(self, other: &Bv<NB>) -> Self {
        let mut s = self;
        s.$op(other, NB);
        s
    }
}
impl <const NB: usize> $tsa for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn $fa(&mut self, other: Bv<NB>) {
        self.$op(& other, NB);
    }
}
impl <const NB: usize> $tra for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn $fa(&mut self, other: &Bv<NB>) {
        self.$op(other, NB);
    }
}
    }}
bit_op!(std::ops::BitOr<Self>, std::ops::BitOr<&Self>, std::ops::BitOrAssign<Self>, std::ops::BitOrAssign<&Self>, bitor, bitor_assign, bit_or);
bit_op!(std::ops::BitAnd<Self>, std::ops::BitAnd<&Self>, std::ops::BitAndAssign<Self>, std::ops::BitAndAssign<&Self>, bitand, bitand_assign, bit_and);
bit_op!(std::ops::BitXor<Self>, std::ops::BitXor<&Self>, std::ops::BitXorAssign<Self>, std::ops::BitXorAssign<&Self>, bitxor, bitxor_assign, bit_xor);
bit_op!(std::ops::Add<Self>, std::ops::Add<&Self>, std::ops::AddAssign<Self>, std::ops::AddAssign<&Self>, add, add_assign, add_msk);
bit_op!(std::ops::Sub<Self>, std::ops::Sub<&Self>, std::ops::SubAssign<Self>, std::ops::SubAssign<&Self>, sub, sub_assign, sub_msk);




//ip Shl/Shr implementations
impl <const NB: usize> std::ops::Shl<usize> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        let mut s = self;
        s.bit_shl::<NB>(rhs);
        s
    }
}

impl <const NB: usize> std::ops::ShlAssign<usize> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn shl_assign(&mut self, rhs: usize) {
        self.bit_shl::<NB>(rhs);
    }
}

impl <const NB: usize> std::ops::Shr<usize> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        let mut s = self;
        s.bit_lshr::<NB>(rhs);
        s
    }
}

impl <const NB: usize> std::ops::ShrAssign<usize> for Bv<NB>
where
    BvN<{ NB }>: IsBv,
{
    fn shr_assign(&mut self, rhs: usize) {
        self.bit_lshr::<NB>(rhs);
    }
}
