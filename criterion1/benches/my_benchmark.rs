use criterion::{criterion_group, criterion_main, Criterion};
use criterion1::my_function;
// use crate::my_function;

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn factorial_benchmarks(c: &mut Criterion) {
    c.bench_function("factorial 20", |b| b.iter(|| factorial(20)));
}

fn my_benchmark(c: &mut Criterion) {
    // ... benchmark ...
    c.bench_function("my_function", |b| b.iter(|| my_function()));
}

criterion_group!(benches, factorial_benchmarks);
// criterion_group!(benches, my_benchmark);
criterion_main!(benches);
