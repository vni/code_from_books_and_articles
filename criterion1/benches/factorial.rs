use criterion::{criterion_group, criterion_main, Criterion};

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn factorial_benchmarks(c: &mut Criterion) {
    c.bench_function("factorial 20", |b| b.iter(|| factorial(20)));
}

criterion_group!(fact_bench, factorial_benchmarks);
criterion_main!(fact_bench);
