#![allow(
    missing_debug_implementations,
    missing_docs,
    clippy::missing_panics_doc
)]

use stacked_set::StackedSet;

#[macro_export]
macro_rules! tests {
    ($tp:ty) => {
        tests!{@ $tp: create_empty, add_single, odd_to_100}
    };
    (@ $tp:ty: $($name:ident),+) => {
        $(#[allow(missing_debug_implementations, missing_docs)]
        #[test]
        pub fn $name() {
            common::$name::<$tp>();
        })*
    }
}

pub fn create_empty<S: StackedSet<Item = i32>>() {
    let s = S::empty();
    assert!(!s.contains(1));
    assert!(!s.contains(2));
    assert!(!s.contains(3));
}

pub fn add_single<S: StackedSet<Item = i32>>() {
    let mut s = S::empty();

    {
        let with_1 = s.extend(1);
        assert!(with_1.contains(1));
        assert!(!with_1.contains(2));
        assert!(!with_1.contains(3));
    }

    {
        let with_2 = s.extend(2);
        assert!(!with_2.contains(1));
        assert!(with_2.contains(2));
        assert!(!with_2.contains(3));
    }

    {
        let with_3 = s.extend(3);
        assert!(!with_3.contains(1));
        assert!(!with_3.contains(2));
        assert!(with_3.contains(3));
    }

    assert!(!s.contains(1));
    assert!(!s.contains(2));
    assert!(!s.contains(3));
}

pub fn odd_to_100<S: StackedSet<Item = i32>>() {
    fn recurse(mut set: impl StackedSet<Item = i32>, val: i32) -> Vec<i32> {
        if val == 0 {
            set.iter().copied().collect()
        } else if val & 1 == 1 {
            recurse(set.extend(val), val - 1)
        } else {
            recurse(set.fork(), val - 1)
        }
    }

    let mut left = recurse(S::empty(), 100);
    left.sort_unstable();
    assert_eq!(
        left,
        (1..=100).filter(|i| *i & 1 == 1).collect::<Vec<i32>>()
    );
}
