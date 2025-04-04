#![allow(missing_docs, missing_debug_implementations)]

use std::any::TypeId;

use criterion::Criterion;
use stacked_set::StackedSet;

pub struct Type<const N: usize>;

macro_rules! fill_stack {
    ($input:ident $output:ident |) => {
        #[allow(unused_mut)]
        let mut $output = $input;
    };
    ($input:ident $output:ident | $int:literal $($rest:literal) *) => {
        #[allow(unused_mut)]
        let mut $input = core::hint::black_box($input.extend(TypeId::of::<Type::<$int>>()));
        fill_stack!($input $output | $($rest) *)
    };
}

macro_rules! bench {
    ($stack:ident $types:literal $c:ident | $($int:literal) *) => {
        $(

            $c.bench_function(
                &format!("c{}_t{}_r:{}", $int, $types, core::any::type_name::<S>()),
                |b| {
                    b.iter(|| {
                        core::hint::black_box($stack.contains(core::hint::black_box(TypeId::of::<Type::<$int>>())));
                    });
                },
            );

            $c.bench_function(
                &format!("nc{}_t{}_r:{}", $int, $types, core::any::type_name::<S>()),
                |b| {
                    b.iter(|| {
                        core::hint::black_box($stack.contains(core::hint::black_box(TypeId::of::<core::marker::PhantomData<Type::<$int>>>())));
                    });
                },
            );
        )*
    };
}

pub fn contains_bench<S: StackedSet<Item = TypeId>>(c: &mut Criterion) {
    let stack = S::empty();
    fill_stack!(stack stack |
    );
    bench!(stack 0 c | 0);

    fill_stack!(stack stack |
        1 2 3 4 5 6 7 8 9 10
    );
    bench!(stack 10 c | 0 10 11);

    fill_stack!(stack stack |
        11 12 13 14 15 16 17 18 19 20
    );
    bench!(stack 20 c | 0 20 21);

    fill_stack!(stack stack |
        21 22 23 24 25 26 27 28 29 30
    );
    bench!(stack 30 c | 0 30 31);

    fill_stack!(stack stack |
        31 32 33 34 35 36 37 38 39 40
    );
    bench!(stack 40 c | 0 40 41);

    fill_stack!(stack stack |
        41 42 43 44 45 46 47 48 49 50
    );
    bench!(stack 50 c | 0 50 51);
}
