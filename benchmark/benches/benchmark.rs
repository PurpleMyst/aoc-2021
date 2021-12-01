use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn day01_benchmark(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::solve()));
}

pub fn alldays_benchmark(c: &mut Criterion) {
    c.bench_function("all", |b| b.iter(|| (day01::solve(),)));
}

criterion_group! {
    name = benches;

    config = Criterion::default()
        .significance_level(0.1)
        .sample_size(500)
        .measurement_time(Duration::from_secs(30))
        .warm_up_time(Duration::from_secs(15))
        .noise_threshold(0.05);

    targets =
        day01_benchmark,
        alldays_benchmark
}

criterion_main!(benches);
