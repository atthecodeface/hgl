//a Idx
//tt Idx
/// The trait for an index
///
/// This *explicitly* does not support `From<usize>`; to create an
/// index type from a usize, use the `from_usize()` method. Supporting
/// From, and hence Into, would weaken the protection provided by the
/// index type, so it *must* be opt-in by the type.
pub trait Idx:
    Copy + std::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash + 'static
{
    fn from_usize(idx: usize) -> Self;
    fn index(self) -> usize;
}
impl Idx for usize {
    fn from_usize(n: usize) -> usize {
        n
    }
    fn index(self) -> usize {
        self
    }
}

//a Macro make_index
#[macro_export]
macro_rules! make_index {
    {$id: ident, $t:ty} => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
        pub struct $id($t);

        impl $crate :: Idx for $id {
            fn from_usize(n: usize) -> Self { Self(n as usize)}
            fn index(self) -> usize {
                self.0 as usize
            }
        }

    }
}
