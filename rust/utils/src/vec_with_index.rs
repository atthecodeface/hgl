//a Imports
use std::collections::HashMap;

use crate::index_vec::{Idx, IndexedVec};

//a IndexKey
//tt IndexKey
pub trait IndexKey: Copy + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static {}

//it IndexKey
impl<T> IndexKey for T where T: Copy + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static {}

//a VecWithIndex
//tp VecWithIndex
/// An [VecWithIndex] is a Vec of items with an index
///
/// When adding
pub struct VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
{
    array: IndexedVec<H, D, false>,
    index: HashMap<K, H>,
}

//ip Default for VecWithIndex<K, H, D>
impl<K, H, D> std::default::Default for VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
{
    fn default() -> Self {
        let array = IndexedVec::default();
        let index = HashMap::default();
        Self { array, index }
    }
}

//ip Index<Handle> for VecWithIndex
impl<K, H, D> std::ops::Index<H> for VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
{
    type Output = D;
    fn index(&self, n: H) -> &D {
        &self.array[n]
    }
}

//ip IntoIter for VecWithIndex
impl<'a, K, H, D> std::iter::IntoIterator for &'a VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
{
    type Item = &'a D;
    type IntoIter = std::slice::Iter<'a, D>;

    // Required method
    fn into_iter(self) -> std::slice::Iter<'a, D> {
        self.array.into_iter()
    }
}

//ip VecWithIndex
impl<'a, K, H, D> VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
{
    //mp find_key
    /// get
    pub fn find_key(&self, key: &K) -> Option<H> {
        self.index.get(key).copied()
    }

    //mp add
    /// Add
    pub fn add(&mut self, key: K, data: D) -> H {
        let index = H::from_usize(self.array.len());
        self.array.push(data);
        self.index.insert(key, index);
        index
    }

    //mp find_or_add
    /// Add
    pub fn find_or_add(&mut self, name: K, data: D) -> H {
        let Some(handle) = self.index.get(&name) else {
            return self.add(name, data);
        };
        *handle
    }
}
//ip Deref for IndexedVec
impl<'a, K, H, D> std::ops::Deref for VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
{
    type Target = IndexedVec<H, D, false>;
    #[inline]
    fn deref(&self) -> &IndexedVec<H, D, false> {
        &self.array
    }
}

//ip AsRef<[D]> for IndexedVec
impl<'a, K, H, D> AsRef<[D]> for VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
{
    #[inline]
    fn as_ref(&self) -> &[D] {
        self.array.as_ref()
    }
}
