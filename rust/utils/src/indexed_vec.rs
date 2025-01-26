//a Imports
use std::marker::PhantomData;

use crate::index_vec::Idx;

//a IndexedVec
//tp IndexedVec
/// An [IndexedVec] is a Vec of items with an index
pub struct IndexedVec<I, T>
where
    I: Idx,
{
    array: Vec<T>,
    _phantom: PhantomData<fn(&I)>,
}

//ip Default for IndexedVec<I, T>
impl<I, T> std::default::Default for IndexedVec<I, T>
where
    I: Idx,
{
    fn default() -> Self {
        let array = vec![];
        Self {
            array,
            _phantom: PhantomData,
        }
    }
}

//ip Index<Handle> for IndexedVec
impl<I, T> std::ops::Index<I> for IndexedVec<I, T>
where
    I: Idx,
{
    type Output = T;
    fn index(&self, idx: I) -> &T {
        &self.array[idx.index()]
    }
}

//ip IntoIter for IndexedVec
impl<'a, I, T> std::iter::IntoIterator for &'a IndexedVec<I, T>
where
    I: Idx,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    // Required method
    fn into_iter(self) -> std::slice::Iter<'a, T> {
        self.array.iter()
    }
}

//ip IndexedVec
impl<I, T> IndexedVec<I, T>
where
    I: Idx,
{
    //ap is_empty
    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    /*
        //ap array
        pub fn array(&self) -> &[T] {
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
        */
}
