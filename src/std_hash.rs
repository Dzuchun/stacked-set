use core::hash::BuildHasher;
use std::collections::HashSet;

use crate::collection::SetCollection;

/// [`std::collections::HashSet`]-based implementation
///
/// On my machine, time to check for existence is flat 10ns
pub type Hash<'l, Item> = crate::collection::CollectionSet<'l, HashSet<Item>>;

impl<Item: Clone + Eq + std::hash::Hash, S: BuildHasher + Default> SetCollection
    for HashSet<Item, S>
{
    type Item = Item;

    type ExtendMemory = Item;

    #[inline]
    fn new() -> Self {
        Self::with_hasher(S::default())
    }

    #[inline]
    fn extend(&mut self, new_item: Self::Item) -> Self::ExtendMemory {
        self.insert(new_item.clone());
        new_item
    }

    #[inline]
    fn contains_ref(&self, item: &Self::Item) -> bool {
        std::collections::HashSet::contains(self, item)
    }

    #[inline]
    fn remove(&mut self, present_item: Self::ExtendMemory) {
        std::collections::HashSet::remove(self, &present_item);
    }

    type IntoIter<'i>
        = std::collections::hash_set::Iter<'i, Item>
    where
        Self: 'i;

    #[inline]
    fn iter(&self) -> Self::IntoIter<'_> {
        std::collections::HashSet::iter(self)
    }
}
