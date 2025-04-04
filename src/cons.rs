use core::{borrow::Borrow, fmt::Debug};

use crate::StackedSet;

/// `Cons list`-like implementation of [`StackedSet`]
///
/// On my machine, worst time to check for existence is about 2ns/item
pub struct ConsSet<'tail, Item>(ConsRepr<'tail, Item>);

// In case you are wondering why is this type private - intend is to hide enum variants from public interface
enum ConsRepr<'tail, Item> {
    Nil,
    Con {
        this: Option<Item>,
        tail: &'tail ConsSet<'tail, Item>,
    },
}

impl<Item: PartialEq + Debug> Debug for ConsSet<'_, Item> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();
        for item in self.iter() {
            list.entry(item);
        }
        list.finish()
    }
}

impl<Item: PartialEq> StackedSet for ConsSet<'_, Item> {
    type Item = Item;

    #[inline]
    fn empty() -> Self {
        ConsSet(ConsRepr::Nil)
    }

    #[inline]
    fn contains_ref(&self, item: &Self::Item) -> bool {
        match &self.0 {
            ConsRepr::Nil => false,
            ConsRepr::Con { this, tail } => {
                if this.as_ref().is_some_and(|this| this == item.borrow()) {
                    true
                } else {
                    tail.contains(item)
                }
            }
        }
    }

    type Shorten<'new>
        = ConsSet<'new, Item>
    where
        Self: 'new;

    #[inline]
    fn extend(&mut self, new_item: Item) -> Self::Shorten<'_> {
        if self.contains_ref(&new_item) {
            ConsSet(ConsRepr::Con {
                this: None,
                tail: self,
            })
        } else {
            ConsSet(ConsRepr::Con {
                this: Some(new_item),
                tail: self,
            })
        }
    }

    #[inline]
    fn fork(&mut self) -> Self::Shorten<'_> {
        ConsSet(ConsRepr::Con {
            this: None,
            tail: self,
        })
    }

    type IntoIter<'i>
        = ConsIter<'i, Item>
    where
        Self: 'i;

    #[inline]
    fn iter(&self) -> Self::IntoIter<'_> {
        ConsIter(&self.0)
    }
}

#[allow(missing_debug_implementations)]
pub struct ConsIter<'l, Item>(&'l ConsRepr<'l, Item>);

impl<'l, Item> Iterator for ConsIter<'l, Item> {
    type Item = &'l Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match core::mem::replace(&mut self.0, &ConsRepr::Nil) {
                ConsRepr::Nil => break None,
                ConsRepr::Con {
                    this: Some(item),
                    tail,
                } => {
                    self.0 = &tail.0;
                    break Some(item);
                }
                ConsRepr::Con { this: None, tail } => {
                    self.0 = &tail.0;
                }
            }
        }
    }
}
