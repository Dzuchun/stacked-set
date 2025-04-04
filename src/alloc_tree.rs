use alloc::collections::BTreeSet;

use crate::collection::SetCollection;

/// [`alloc::collections::BTreeSet`]-based implementation
///
/// On my machine, worst time to check for existence is about
/// - 13 ns for 10
/// - 18 ns for 20
/// - <=20 ns for 30
/// - <=20 ns for 40
/// - <=20 ns for 50
pub type TreeSet<'l, Item> =
    crate::collection::CollectionSet<'l, alloc::collections::BTreeSet<Item>>;

impl<Item: Ord + PartialEq + Clone> SetCollection for BTreeSet<Item> {
    type Item = Item;

    type ExtendMemory = Item;

    #[inline]
    fn new() -> Self {
        Self::new()
    }

    #[inline]
    fn extend(&mut self, new_item: Self::Item) -> Self::ExtendMemory {
        self.insert(new_item.clone());
        new_item
    }

    #[inline]
    fn contains_ref(&self, item: &Self::Item) -> bool {
        alloc::collections::BTreeSet::contains(self, item)
    }

    #[inline]
    fn remove(&mut self, present_item: Self::ExtendMemory) {
        alloc::collections::BTreeSet::remove(self, &present_item);
    }
}
