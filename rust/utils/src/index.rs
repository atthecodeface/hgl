//a Idx
//tt Idx
pub trait Idx:
    Copy + std::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash + 'static
{
    fn from_usize(idx: usize) -> Self;
    fn index(self) -> usize;
}

//a Macro make_index
#[macro_export]
macro_rules! make_index {
    {$id: ident, $t:ty} => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
        pub struct $id($t);

        impl $crate :: index_vec :: Idx for $id {
            fn from_usize(n: usize) -> Self { Self(n as usize)}
            fn index(self) -> usize {
                self.0 as usize
            }
        }

    }
}
