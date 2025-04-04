use crate::collection::SetCollection;

/// [`alloc::vec::Vec`]-based implementation
///
/// On my machine, worst time to check for existence is about 0.8ns/item
pub type Vec<'l, Item> = crate::collection::CollectionSet<'l, alloc::vec::Vec<Item>>;

impl<Item: PartialEq> SetCollection for alloc::vec::Vec<Item> {
    type Item = Item;

    type ExtendMemory = ();

    #[inline]
    fn new() -> Self {
        alloc::vec::Vec::new()
    }

    #[inline]
    fn extend(&mut self, new_item: Self::Item) -> Self::ExtendMemory {
        self.push(new_item);
    }

    #[inline]
    fn contains_ref(&self, item: &Self::Item) -> bool {
        <[Item]>::contains(self, item)
    }

    #[inline]
    fn remove(&mut self, _present_item: Self::ExtendMemory) {
        let _ = self.pop();
    }
}
