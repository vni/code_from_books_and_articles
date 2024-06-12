use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Instant;

static A: AtomicU64 = AtomicU64::new(0);

fn main() {
    let start = Instant::now();

    for _ in 0 .. 1_000_000_000 {
        A.load(Relaxed);
    }
    println!("{:?}", start.elapsed());
}
