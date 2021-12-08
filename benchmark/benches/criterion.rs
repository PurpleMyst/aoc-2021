use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn day01_benchmark(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::solve()));
}

pub fn day02_benchmark(c: &mut Criterion) {
    c.bench_function("day02", |b| b.iter(|| day02::solve()));
}

pub fn day03_benchmark(c: &mut Criterion) {
    c.bench_function("day03", |b| b.iter(|| day03::solve()));
}

pub fn day04_benchmark(c: &mut Criterion) {
    c.bench_function("day04", |b| b.iter(|| day04::solve()));
}

pub fn day05_benchmark(c: &mut Criterion) {
    c.bench_function("day05", |b| b.iter(|| day05::solve()));
}

pub fn day06_benchmark(c: &mut Criterion) {
    c.bench_function("day06", |b| b.iter(|| day06::solve()));
}

pub fn day07_benchmark(c: &mut Criterion) {
    c.bench_function("day07", |b| b.iter(|| day07::solve()));
}

pub fn day08_benchmark(c: &mut Criterion) {
    c.bench_function("day08", |b| b.iter(|| day08::solve()));
}

pub fn alldays_benchmark(c: &mut Criterion) {
    c.bench_function("all", |b| {
        b.iter(|| {
            (
                day01::solve(),
                day02::solve(),
                day03::solve(),
                day04::solve(),
                day05::solve(),
                day06::solve(),
                day07::solve(),
                day08::solve(),
            )
        })
    });
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
        day02_benchmark,
        day03_benchmark,
        day04_benchmark,
        day05_benchmark,
        day06_benchmark,
        day07_benchmark,
        day08_benchmark,
        alldays_benchmark
}

criterion_main!(benches);
