use std::thread;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::hint::black_box;

// #[repr(align(64))]
// #[repr(align(128))]
#[repr(align(256))]
struct Aligned(AtomicU64);

/*  No false sharing effect. */

static A: [Aligned; 3] = [
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
];

fn main() {
    black_box(&A);

    thread::spawn(|| {
        loop {
            A[0].0.store(0, Relaxed);
            A[2].0.store(0, Relaxed);
        }
    });

    let start = std::time::Instant::now();
    for _ in 0 .. 1_000_000_000 {
        black_box(A[1].0.load(Relaxed));
    }

    println!("{:?}", start.elapsed());
}
