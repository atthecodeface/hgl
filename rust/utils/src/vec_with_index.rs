//a Imports
use std::collections::HashMap;

use crate::index_vec::Idx;

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
    array: Vec<D>,
    index: HashMap<K, H>,
}

//ip Default for VecWithIndex<K, H, D>
impl<K, H, D> std::default::Default for VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
{
    fn default() -> Self {
        let array = vec![];
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
        &self.array[n.index()]
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
        self.array.iter()
    }
}

//ip VecWithIndex
impl<'a, K, H, D> VecWithIndex<K, H, D>
where
    K: IndexKey,
    H: Idx,
{
    //ap is_empty
    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    //ap array
    pub fn array(&self) -> &[D] {
        &self.array
    }

    //mp first
    /// get
    ///
    /// FIXME
    pub fn first(&self) -> Option<&D> {
        Some(&self.array[0]) // H::from_usize(0)])
    }

    //mp get
    /// get
    pub fn get(&self, key: &K) -> Option<H> {
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
