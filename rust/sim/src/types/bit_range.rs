use crate::types::U8Ops;

pub struct BitRange<'a, D>
where
    [D]: U8Ops,
{
    lsb: usize,
    n: usize,
    data: &'a [D],
}
impl<'a, D> BitRange<'a, D>
where
    [D]: U8Ops,
{
    pub fn of_u8s(data: &'a [D], lsb: usize, n: usize) -> Self {
        Self { lsb, n, data }
    }
}

pub struct BitRangeMut<'a, D>
where
    [D]: U8Ops,
{
    lsb: usize,
    n: usize,
    data: &'a mut [D],
}
impl<'a, D> BitRangeMut<'a, D>
where
    [D]: U8Ops,
{
    pub fn of_u8s(data: &'a mut [D], lsb: usize, n: usize) -> Self {
        Self { lsb, n, data }
    }
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
