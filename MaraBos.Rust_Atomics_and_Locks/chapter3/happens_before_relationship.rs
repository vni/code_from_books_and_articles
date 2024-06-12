use std::sync::atomic::{AtomicI32, Ordering::Relaxed};
use std::thread;
use std::time::Duration;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.store(10, Relaxed);
    // thread::sleep(Duration::from_nanos(1));
    Y.store(20, Relaxed);
}

fn b() {
    let y = Y.load(Relaxed);
    // thread::sleep(Duration::from_nanos(1));
    let x = X.load(Relaxed);
    println!("{x} {y}");

    // I'm awaiting to see 10 20, 0 20, 10 0 and 0 0 variants
}

fn main() {
    thread::scope(|scope| {
        scope.spawn(a);
        scope.spawn(b);
    });
}
