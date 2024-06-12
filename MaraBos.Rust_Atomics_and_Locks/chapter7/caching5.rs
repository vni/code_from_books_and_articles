use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Instant;
use std::thread;

static A: AtomicU64 = AtomicU64::new(0);

/*
Because A is always zero, this compare_exchange operation will never succeed. It’ll load the current value of A, but never update it to a new value.

One might reasonably expect this to behave the same as a load operation, since it does not modify the atomic variable. However, on most processor architectures, the instruction(s) of compare_exchange will claim exclusive access of the relevant cache line regardless of whether the comparison succeeds or not.

This means that it can be beneficial to not use compare_exchange (or swap) in a spin loop like we did for our SpinLock in Chapter 4, but instead use a load operation first to check if the lock has been unlocked. That way, we avoid unnecessarily claiming exclusive access to the relevant cache line.
 */

fn main() {
    black_box(&A);

    thread::spawn(|| {
        loop {
            // Never succeeds, because A is never 10.
            black_box(A.compare_exchange(10, 20, Relaxed, Relaxed).is_ok());
        }
    });

    let start = Instant::now();
    for _ in 0 .. 1_000_000_000 {
        black_box(A.load(Relaxed));
    }
    println!("{:?}", start.elapsed());
}
