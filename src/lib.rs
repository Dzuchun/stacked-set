#![no_std] // <-- see that attr? no shit!
#![doc = include_str!("../README.md")]

use core::borrow::Borrow;

#[cfg(feature = "collection")]
#[doc(hidden)]
mod collection;

#[cfg(feature = "cons")]
#[doc(hidden)]
mod cons;
/// Common trait for stacked set implementations. Users are intended to define their input as `impl StackedSet<Item = WhateverItemTheyNeed>`, so it's up to the user to pick the implementation
pub trait StackedSet: Sized {
    /// Item stored in the set
    type Item;

    /// Creates an empty set
    #[must_use = "Creating empty set is usually a no-op"]
    fn empty() -> Self;

    /// Checks if element is present in the set
    fn contains(&self, item: impl Borrow<Self::Item>) -> bool {
        self.contains_ref(item.borrow())
    }

    /// Checks if element is present in the set
    #[must_use = "Checking for presence does not change set contents"]
    fn contains_ref(&self, item: &Self::Item) -> bool;

    /// Intended to be the same type, but living for less time
    type Shorten<'new>: StackedSet<Item = Self::Item> + 'new
    where
        Self: 'new;

    /// Adds an item to the stack, returning a new instance now "containing" the item
    ///
    /// Note, that original instance is being exclusively borrowed, i.e.
    /// - new stacked set should not outlive it's parent
    /// - parent itself cannot be observed while new instance lives
    ///
    /// Once this new instance is dropped, original stack is not supposed to contain this new type
    #[must_use = "Provided value is only contained in a set returned form this call. Despite requiring exclusive borrow, original set is should not be expected to change. Check documentation for more details."]
    fn extend(&mut self, new_item: Self::Item) -> Self::Shorten<'_>;

    /// Same as [`StackedSet::extend`], but does not actually extend the stackset
    ///
    /// Intended to be used, when you need to pass [`StackedSet`] implementor into multiple inner calls, while retaining ownership of the original one
    #[must_use = "Despite requiring exclusive borrow, original set is should not be expected to change. Check documentation for more details."]
    fn fork(&mut self) -> Self::Shorten<'_>;
}

#[cfg(feature = "cons")]
pub use cons::ConsSet as StackCons;

#[cfg(feature = "collection")]
pub use collection::SetCollection;

#[cfg(test)]
mod tests;
