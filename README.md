```text
This crate does not require nor std nor alloc ❤️
```

# Stacked set

This crate defines a so-called "stacked set" - a set-like interface that allows implementations without dynamic memory.

Interface itself can be summarized by [`StackedSet`] trait:
```rust,ignore
// (simplified)
trait StackedSet {
    fn empty() -> Self;
    fn contains(&self, item: impl Borrow<Self::Item>) -> bool;
    fn extend(&mut self, new_item: Self::Item) -> Self; // (output is not really `Self`, but that's the basic idea)
    fn fork(&mut self) -> Self; // (same here, it's not actually `Self`)
}
```

## Picking the implementation

Currently, 4 implementations are provided:
- cons-like alloc-free implementation
- `Vec`-based implementation (needs `alloc`)
- `BTreeSet`-based implementation (needs `alloc`)
- `HashSet`-based implementation (needs `std::hash`)

All of them are feature-locked and `cons` implementation is the only one enabled by default.

## Usage example

Creating:
```rust
use stacked_set::{
    StackedSet,
    StackCons, // <- this implementation specifically is alloc-free!
};
let set = StackCons::<'static, i32>::empty();
```

Don't let random `'static` lifetime confuse you - it means that this stacked set in particular has no dependency on any other stacked set below it. That is indeed correct, as empty set contains nothing.

Let's add a `1` to the set:
```rust
# use stacked_set::{StackCons, StackedSet};
# let mut set = StackCons::<'static, i32>::empty();
let with_1 = set.extend(1);
```

Now `1` is present in the set, while `2` is not:

```rust
# use stacked_set::{StackCons, StackedSet};
# let mut set = StackCons::<'static, i32>::empty();
# let with_1 = set.extend(1);
assert!(with_1.contains(1));
assert!(!with_1.contains(&2));
```

Note that both `i32` and `&i32` were passed here - `contains` method of this set accepts `impl Borrow<i32>`.

**NOTE**: you *cannot* use `set`, with `with_1` is alive:

```rust,compile_fail
# use stacked_set::{StackCons, StackedSet};
# let mut set = StackCons::<'static, i32>::empty();
# let with_1 = set.extend(1);
set.contains(42);
&with_1;
```

This happens, because internally `with_1` is thought to contain an exclusive borrow of `set`.

This part of design has nothing to do with stack-only implementation, but allows implementations with normal kind of collections, like `Vec` (see [Collection trait](#setcollection-trait) below)

To remove 1 from the set, drop `with_1` handle:

```rust
# use stacked_set::{StackCons, StackedSet};
# let mut set = StackCons::<'static, i32>::empty();
# let with_1 = set.extend(1);
let _ = with_1;
// no more 1 in the set!
assert!(!set.contains(1));
```

**NOTE**: this means that stacked set cannot be used as "out parameter":

```rust,should_panic
# use stacked_set::{StackCons, StackedSet};
fn add_1(set: &mut impl StackedSet<Item = i32>) {
    // add 1
    set.extend(1);
}
let mut set = StackCons::<'static, i32>::empty();
add_1(&mut set);
// doesn't work :(
assert!(set.contains(1)); // <-- panic occurs here
```

You can return the set itself though:

```rust
# use stacked_set::{StackCons, StackedSet};
fn append_1<S: StackedSet<Item = i32>>(set: &mut S) -> impl StackedSet<Item = i32> + use<'_, S> {
    set.extend(1)
}
# let mut set = StackCons::<'static, i32>::empty();
let with_1 = append_1(&mut set);
// it works! :3
assert!(with_1.contains(1)); // <-- Rust no panic: Rust happy
core::mem::drop(with_1); // <-- must drop this variable to be able to use `set`
assert!(!set.contains(1));
```

Please note however, that you can't return more than 1 value in this way:

```rust,compile_fail
# use stacked_set::{StackCons, StackedSet};
fn append_1_and_2<S: StackedSet<Item = i32>>(set: &mut S) -> impl StackedSet<Item = i32> + use<'_, S> {
    let mut with_1 = set.extend(1)
    let with_1_and_2 = with_1.extend(2);
    with_1_and_2 // <-- can't return, as it references temporary value (`with_1`)
}
# let mut set = StackCons::<'static, i32>::empty();
let _ = append_1_and_2(&mut set);
```

So `StackedSet`s are not suited well for this kind of task.

Instead, consider passing values to inner calls:

```rust
# use stacked_set::{StackCons, StackedSet};
fn nested(mut set: impl StackedSet<Item = i32>, val: i32) {
    if val == 0 {
        for i in 1..=10 {
            assert!(set.contains(i));
        }
        assert!(!set.contains(11));
        assert!(!set.contains(0));
    } else {
        nested(set.extend(val), val-1);
    }
}
let mut set = StackCons::empty();
nested(set, 10);
```

(this is, in fact, the intended usecase)

You can also iterate over values in the set:

```rust
# use stacked_set::{StackCons, StackedSet};
fn nested(mut set: impl StackedSet<Item = i32>, val: i32) {
    if val == 0 {
        let mut v = set.iter().copied().collect::<Vec<_>>();
        v.sort_unstable();
        assert_eq!(v, vec![1, 3, 5, 7, 9]);
    } else if val & 1 == 1 {
        nested(set.extend(val), val-1);
    } else {
        nested(set.fork(), val-1);
    }
}
let mut set = StackCons::empty();
nested(set, 10);
```

## `SetCollection` trait

`collection` feature locks the `SetCollection` trait:
```rust,ignore
(simplified)
pub trait SetCollection {
    type ExtendMemory;
    fn new() -> Self;
    fn extend(&mut self, new_item: Self::Item) -> Self::ExtendMemory;
    fn contains_ref(&self, item: &Self::Item) -> bool;
    fn remove(&mut self, present_item: Self::ExtendMemory);
}
```

Note that this trait can be pretty easily implemented for normal kind of collection, like `Vec` or `HashSet`, an that's exactly what they implement, actually. User is not intended to use this trait directly. Instead, use `CollectionSet` wrapper to convert `Vec`, `BTreeSet` or `HashSet` into `StackedSet` implementation. Exported variants of `CollectionSet` can be found in this crate.

## `StackedSet` trait

If `StackedSet` need to be implemented, here's a bit of explanation on `Shorten`:
Stacked sets are intended to borrow other instances of their type, but technically shorter lifetime makes it a different type. There's no good way to represent that, so in case of this trait, it's defined as `CollectionSet::Shorten`. Basically, `Shorten` should be assigned a `Self` type, but with a lifetime provided by associated type definition.

Another thing to keep in mind is that `&mut T` is *invariant* over lifetime, so some sort of type surgery is usually required during implementation.

Also please remember that each instance of `StackedSet` is responsible for removing it's own element from the set. This is usually done via `Drop` implementation, but it's not necessarily required - for example, `StackCons` does not need it.
