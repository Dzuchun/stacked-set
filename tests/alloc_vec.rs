#![allow(missing_docs)]
#![cfg(feature = "alloc-vec")]

mod common;

tests!(stacked_set::AllocVec::<'static, i32>);
