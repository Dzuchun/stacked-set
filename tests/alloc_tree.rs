#![allow(missing_docs)]
#![cfg(feature = "alloc-tree")]

mod common;

tests!(stacked_set::AllocTree::<'static, i32>);
