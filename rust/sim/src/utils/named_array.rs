//a Imports
use std::collections::HashMap;

use crate::traits::{Index, Key};

//a Macro make_handle
#[macro_export]
macro_rules! make_handle {
    {$id: ident} => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
        pub struct $id(usize);

        impl From<usize> for $id {
            fn from(n: usize) -> Self {
                Self(n)
            }
        }

        impl  $crate :: traits :: Index for $id {
            fn index(self) -> usize {
                self.0
            }
        }
    }
}

//a Array
//tp Array
pub struct Array<N, H, D>
where
    N: Key,
    H: Index,
{
    array: Vec<D>,
    index: HashMap<N, H>,
}

//ip Default for Array<N, H, D>
impl<N, H, D> std::default::Default for Array<N, H, D>
where
    N: Key,
    H: Index,
{
    fn default() -> Self {
        let array = vec![];
        let index = HashMap::default();
        Self { array, index }
    }
}

//ip Index<Handle> for Array
impl<N, H, D> std::ops::Index<H> for Array<N, H, D>
where
    N: Key,
    H: Index,
{
    type Output = D;
    fn index(&self, n: H) -> &D {
        &self.array[n.index()]
    }
}

//ip IntoIter for Array
impl<'a, N, H, D> std::iter::IntoIterator for &'a Array<N, H, D>
where
    N: Key,
    H: Index,
{
    type Item = &'a D;
    type IntoIter = std::slice::Iter<'a, D>;

    // Required method
    fn into_iter(self) -> std::slice::Iter<'a, D> {
        self.array.iter()
    }
}

//ip Array
impl<'a, N, H, D> Array<N, H, D>
where
    N: Key,
    H: Index,
{
    //ap is_empty
    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    //ap array
    pub fn array(&self) -> &[D] {
        &self.array
    }

    //mp get
    /// get
    pub fn get(&self, name: &N) -> Option<H> {
        self.index.get(name).copied()
    }

    //mp add
    /// Add
    pub fn add(&mut self, name: N, data: D) -> H {
        let handle = self.array.len().into();
        self.array.push(data);
        self.index.insert(name, handle);
        handle
    }
    //mp find_or_add
    /// Add
    pub fn find_or_add(&mut self, name: N, data: D) -> H {
        let Some(handle) = self.index.get(&name) else {
            return self.add(name, data);
        };
        *handle
    }
}
