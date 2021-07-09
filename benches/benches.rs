use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod day_09;

// TODO: Better benchmarks.
// Use my Day 9 input from 2019's Advent of Code as our benchmark
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day 9, Part 1", |b| {
        b.iter(|| black_box(day_09::part_1(black_box(1024))))
    });
    c.bench_function("Day 9, Part 2", |b| {
        b.iter(|| black_box(day_09::part_2(black_box(1024))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
