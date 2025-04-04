use core::any::TypeId;

use crate::StackedSet;

#[allow(unused)]
type T1 = [(); 1];
#[allow(unused)]
type T2 = [(); 2];
#[allow(unused)]
type T3 = [(); 3];
#[allow(unused)]
type T4 = [(); 4];
#[allow(unused)]
type T5 = [(); 5];

#[allow(unused)]
type T1C = [(); 1];

macro_rules! contain {
    ($stack:ident, $type:ty) => {
        assert!(
            $stack.contains(TypeId::of::<$type>()),
            "Stack should contain {} at this point",
            core::any::type_name::<$type>()
        );
    };
}

macro_rules! not_contain {
    ($stack:ident, $type:ty) => {
        assert!(
            !$stack.contains(TypeId::of::<$type>()),
            "Stack should not contain {} at this point",
            core::any::type_name::<$type>()
        );
    };
}

#[inline]
#[allow(unused)]
fn test_impl<S: StackedSet<Item = TypeId>>() {
    let mut stack = S::empty();
    not_contain!(stack, T1);
    not_contain!(stack, T2);
    not_contain!(stack, T3);
    not_contain!(stack, T4);
    not_contain!(stack, T5);
    not_contain!(stack, T1C);
    // add type 1
    {
        let mut stack = stack.extend(TypeId::of::<T1>());
        contain!(stack, T1);
        not_contain!(stack, T2);
        not_contain!(stack, T3);
        not_contain!(stack, T4);
        not_contain!(stack, T5);
        contain!(stack, T1C);
    }
    // removed type 1
    not_contain!(stack, T1);
    not_contain!(stack, T2);
    not_contain!(stack, T3);
    not_contain!(stack, T4);
    not_contain!(stack, T5);
    not_contain!(stack, T1C);
    // add type 2
    {
        let mut stack = stack.extend(TypeId::of::<T2>());
        not_contain!(stack, T1);
        contain!(stack, T2);
        not_contain!(stack, T3);
        not_contain!(stack, T4);
        not_contain!(stack, T5);
        not_contain!(stack, T1C);
        // add type 2 once again
        {
            let mut stack = stack.extend(TypeId::of::<T2>());
            not_contain!(stack, T1);
            contain!(stack, T2);
            not_contain!(stack, T3);
            not_contain!(stack, T4);
            not_contain!(stack, T5);
            not_contain!(stack, T1C);
        }
        // type 2 should still be present
        not_contain!(stack, T1);
        contain!(stack, T2);
        not_contain!(stack, T3);
        not_contain!(stack, T4);
        not_contain!(stack, T5);
        not_contain!(stack, T1C);
    }
    // remove type 2
    not_contain!(stack, T1);
    not_contain!(stack, T2);
    not_contain!(stack, T3);
    not_contain!(stack, T4);
    not_contain!(stack, T5);
    not_contain!(stack, T1C);
    // add all types
    {
        let mut stack = stack.extend(TypeId::of::<T1>());
        let mut stack = stack.extend(TypeId::of::<T2>());
        let mut stack = stack.extend(TypeId::of::<T3>());
        let mut stack = stack.extend(TypeId::of::<T4>());
        let mut stack = stack.extend(TypeId::of::<T5>());
        contain!(stack, T1);
        contain!(stack, T2);
        contain!(stack, T3);
        contain!(stack, T4);
        contain!(stack, T5);
        contain!(stack, T1C);
    }
    // remove all types
    not_contain!(stack, T1);
    not_contain!(stack, T2);
    not_contain!(stack, T3);
    not_contain!(stack, T4);
    not_contain!(stack, T5);
    not_contain!(stack, T1C);
}

#[allow(unused)]
macro_rules! test_impl {
    ($name:ident, $type:ty) => {
        #[test]
        fn $name() {
            test_impl::<$type>();
        }
    };
}

#[cfg(feature = "cons")]
test_impl!(cons, crate::StackCons<'static, TypeId>);

#[cfg(feature = "alloc-vec")]
test_impl!(alloc_vec, crate::AllocVec<'static, TypeId>);

#[cfg(feature = "alloc-tree")]
test_impl!(alloc_tree, crate::AllocTree<'static, TypeId>);
