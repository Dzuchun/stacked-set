#![allow(missing_docs)]
#![cfg(feature = "std-hash")]

mod common;

tests!(stacked_set::StdHash::<'static, i32>);
