use criterion::{criterion_group, criterion_main, Criterion};

fn fibonacci1(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci1(n - 1) + fibonacci1(n - 2),
    }
}

fn fibonacci2(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("My Group");
    group.bench_function("fibonacci1(30)", |b| b.iter(|| fibonacci1(30)));
    group.bench_function("fibonacci2(30)", |b| b.iter(|| fibonacci2(30)));
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
