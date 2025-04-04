use core::{borrow::Borrow, fmt::Debug, ops::Deref};

use crate::StackedSet;

/// Kind of interface a "collection" should expose for [`StackedSet`] implementation to be built on top of it.
///
/// To get a [`StackedSet`] implementor, just wrap your collection into [`CollectionSet`]. Original collection will still be available via `Deref`/`Borrow`/`AsRef`.
pub trait SetCollection {
    /// Element type stored in the collection.
    type Item;

    /// A some sort of memory that can be used to remove the item back. Some implementations (like `Vec` may want to make this a unit type, if the item to remove is somehow known through collection structure).
    type ExtendMemory;

    /// Creates an empty collection.
    fn new() -> Self;

    /// Extends the collection, creating instance of `ExtendMemory` to later remove this element. Note that implementation should not care about item previously existing, as [`CollectionSet`] checks for `new_item` not being present in the collection prior to this call.
    fn extend(&mut self, new_item: Self::Item) -> Self::ExtendMemory;

    /// Checks if the collection contains a certain element. Only accepts in a core reference, check [`SetCollection::contains`] method if more flexibility is needed.
    fn contains_ref(&self, item: &Self::Item) -> bool;

    /// Checks if the collection contains a certain element.
    #[inline]
    fn contains(&self, item: impl Borrow<Self::Item>) -> bool {
        self.contains_ref(item.borrow())
    }

    /// Removes an element from the collection represented by [`SetCollection::ExtendMemory`] instance.
    fn remove(&mut self, present_item: Self::ExtendMemory);

    /// Type of iterator over item references.
    type IntoIter<'i>: Iterator<Item = &'i Self::Item>
    where
        Self: 'i;

    /// Creates iterator over item references.
    fn iter(&self) -> Self::IntoIter<'_>;
}

/// [`SetCollection`]-based implementation.
///
/// On my machine, worst time to check for existence is about 0.6ns/item.
pub struct CollectionSet<'l, Collection: SetCollection>(CollectionRepr<'l, Collection>);

// In case you are wondering why is this type private - intend is to hide enum variants from public interface
enum CollectionRepr<'l, Collection: SetCollection> {
    Nil(Collection),
    Fork(&'l mut Collection),
    Extend(&'l mut Collection, Collection::ExtendMemory),
    Moved,
}

impl<Collection: SetCollection> CollectionSet<'_, Collection> {
    /// A private method for convenient collection mutation
    #[inline]
    pub(self) fn c_mut(&mut self) -> &mut Collection {
        match &mut self.0 {
            CollectionRepr::Nil(c) => c,
            CollectionRepr::Fork(c) | CollectionRepr::Extend(c, _) => c,
            CollectionRepr::Moved => unreachable!("Cannot call instance method after drop"),
        }
    }
}

impl<Collection: SetCollection> Deref for CollectionSet<'_, Collection> {
    type Target = Collection;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match &self.0 {
            CollectionRepr::Nil(c) => c,
            CollectionRepr::Fork(c) | CollectionRepr::Extend(c, _) => c,
            CollectionRepr::Moved => unreachable!("Cannot call instance method after drop"),
        }
    }
}

impl<Collection: SetCollection> Borrow<Collection> for CollectionSet<'_, Collection> {
    #[inline]
    fn borrow(&self) -> &Collection {
        self
    }
}

impl<Collection: SetCollection> AsRef<Collection> for CollectionSet<'_, Collection> {
    #[inline]
    fn as_ref(&self) -> &Collection {
        self
    }
}

impl<Collection: SetCollection + Debug> Debug for CollectionSet<'_, Collection> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Collection as core::fmt::Debug>::fmt(&**self, f)
    }
}

impl<Collection: SetCollection> Drop for CollectionSet<'_, Collection> {
    #[inline]
    fn drop(&mut self) {
        let repr = core::mem::replace(&mut self.0, CollectionRepr::Moved);
        if let CollectionRepr::Extend(c, m) = repr {
            c.remove(m);
        }
    }
}

impl<Collection: SetCollection> StackedSet for CollectionSet<'_, Collection> {
    type Item = Collection::Item;

    #[inline]
    fn empty() -> Self {
        Self(CollectionRepr::Nil(Collection::new()))
    }

    #[inline]
    fn contains_ref(&self, item: &Self::Item) -> bool {
        Collection::contains_ref(self, item)
    }

    type Shorten<'new>
        = CollectionSet<'new, Collection>
    where
        Self: 'new;

    #[inline]
    fn extend(&mut self, new_item: Self::Item) -> Self::Shorten<'_> {
        if self.contains_ref(&new_item) {
            CollectionSet(CollectionRepr::Fork(self.c_mut()))
        } else {
            let m = self.c_mut().extend(new_item);
            CollectionSet(CollectionRepr::Extend(self.c_mut(), m))
        }
    }

    #[inline]
    fn fork(&mut self) -> Self::Shorten<'_> {
        CollectionSet(CollectionRepr::Fork(self.c_mut()))
    }

    type IntoIter<'i>
        = Collection::IntoIter<'i>
    where
        Self: 'i;

    #[inline]
    fn iter(&self) -> Self::IntoIter<'_> {
        let c: &Collection = self;
        c.iter()
    }
}
