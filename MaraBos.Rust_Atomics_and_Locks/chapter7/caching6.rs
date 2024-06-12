use std::thread;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::hint::black_box;

/*  This effect is called false sharing. */

static A: [AtomicU64; 3] = [
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
];

fn main() {
    black_box(&A);

    thread::spawn(|| {
        loop {
            A[0].store(0, Relaxed);
            A[2].store(0, Relaxed);
        }
    });

    let start = std::time::Instant::now();
    for _ in 0 .. 1_000_000_000 {
        black_box(A[1].load(Relaxed));
    }

    println!("{:?}", start.elapsed());
}
