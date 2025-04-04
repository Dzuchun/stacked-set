#![allow(
    missing_debug_implementations,
    missing_docs,
    clippy::missing_panics_doc
)]

use stacked_set::StackedSet;

#[macro_export]
macro_rules! tests {
    ($tp:ty) => {
        tests!{@ $tp: create_empty, add_single}
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
