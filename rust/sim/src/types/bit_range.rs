//a Imports
use crate::types::U8Ops;


//a BitRange
//tp BitRange
/// A selections of bits from within a slice (e.g. of u8)
pub struct BitRange<'a, D>
where
    [D]: U8Ops,
{
    lsb: usize,
    n: usize,
    data: &'a [D],
}

//ip BitRange
impl<'a, D> BitRange<'a, D>
where
    [D]: U8Ops,
{
    //cp of_u8s
    /// Create a BitRange from provided data
    ///
    /// lsb + n should be less than the size in bits of data
    pub fn of_u8s(data: &'a [D], lsb: usize, n: usize) -> Self {
        Self { lsb, n, data }
    }
}

//ip From<BitRangeMut> for BitRange
impl <'a, D> From<BitRangeMut<'a, D>> for BitRange<'a, D>
where
    [D]: U8Ops,
{
    fn from(f:BitRangeMut<'a, D>) -> BitRange<'a, D> {
        Self {
            lsb: f.lsb,
            n: f.n,
            data: f.data
        }
    }
}

//a BitRangeMut
//tp BitRangeMut
/// A mutable selection of bits from within a slice
pub struct BitRangeMut<'a, D>
where
    [D]: U8Ops,
{
    lsb: usize,
    n: usize,
    data: &'a mut [D],
}

//ip BitRangeMut
impl<'a, D> BitRangeMut<'a, D>
where
    [D]: U8Ops,
{
    //cp of_u8s
    /// Create a BitRangeMut from a given data slice and bits
    ///
    /// lsb + n should be less than the size in bits of data
    pub fn of_u8s(data: &'a mut [D], lsb: usize, n: usize) -> Self {
        Self { lsb, n, data }
    }

    //mp set
    /// Set the bits to a BitRange
    #[track_caller]
    pub fn set<const NB: usize>(&mut self, other: BitRange<'a, D>) {
        assert_eq!(
            self.n, other.n,
            "Assignment of bit range has mismatched widths"
        );
        for i in 0..self.n {
            let b = other.data.bit::<NB>(other.lsb + i);
            self.data.bit_set::<NB>(self.lsb + i, b);
        }
    }
}
