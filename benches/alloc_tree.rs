#![allow(missing_docs)]

use std::{any::TypeId, time::Duration};

use common::contains_bench;
use criterion::{Criterion, criterion_group, criterion_main};
use stacked_set::AllocTree;

mod common;

criterion_group! {
    name = contains;
    config = Criterion::default().sample_size(2000).warm_up_time(Duration::from_millis(500)).measurement_time(Duration::from_secs(5)).with_plots();
    targets = contains_bench::<AllocTree<'static, TypeId>>
}
criterion_main!(contains);
