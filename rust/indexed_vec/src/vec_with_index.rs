//a Imports
use std::collections::HashMap;

use crate::{Idx, IndexedVec};

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

//ip Debug for VecWithIndex<K, H, D>
impl<K, H, D> std::fmt::Debug for VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
    D: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "VecWInd[")?;
        let mut first = true;
        for (_k, h) in self.index.iter() {
            if !first {
                fmt.write_str(", ")?;
            }
            let d = &self.array[*h];
            write!(fmt, "{h:?}: {d:?}")?;
            first = false;
        }
        write!(fmt, "]")
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

//ip Deref for VecWithIndex
impl<K, H, D> std::ops::Deref for VecWithIndex<K, H, D>
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

//ip AsRef<[D]> for VecWithIndex
impl<K, H, D> AsRef<[D]> for VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
{
    #[inline]
    fn as_ref(&self) -> &[D] {
        self.array.as_ref()
    }
}

//ip VecWithIndex
impl<K, H, D> VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
{
    //mp find_key
    /// Find the key in the index, if it is there
    pub fn find_key(&self, key: &K) -> Option<H> {
        self.index.get(key).copied()
    }

    //mp find_or_add
    /// Add data to the array and index, but only if it is not present
    ///
    /// Returns (found, index) - i.e. found is true if the key was
    /// already present, and false if the key was not present and the
    /// data was added to the array
    pub fn find_or_add<F: FnOnce(&K) -> D>(&mut self, key: K, f: F) -> (bool, H) {
        if let Some(index) = self.index.get(&key) {
            (true, *index)
        } else {
            let index = self.array.push(f(&key));
            self.index.insert(key, index);
            (false, index)
        }
    }

    //mp insert
    /// Add data to the array and index, but only if it is not present
    ///
    /// If it is already present, return an Err
    pub fn insert<F: FnOnce(&K) -> D>(&mut self, key: K, f: F) -> Result<H, H> {
        if let Some(index) = self.index.get(&key) {
            Err(*index)
        } else {
            let index = self.array.push(f(&key));
            self.index.insert(key, index);
            Ok(index)
        }
    }
}
