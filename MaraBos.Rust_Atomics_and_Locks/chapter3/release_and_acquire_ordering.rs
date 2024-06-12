use std::sync::atomic::{AtomicU64, AtomicBool, Ordering::{Acquire, Release, Relaxed}};
use std::time::Duration;
use std::thread;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    let t = thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release);
    });

    while !READY.load(Acquire) {
        thread::sleep(Duration::from_nanos(1));
        println!("waiting...");
    }
    println!("{}", DATA.load(Relaxed));

    t.join().unwrap();
}
