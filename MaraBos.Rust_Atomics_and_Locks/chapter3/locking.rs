#[allow(unused_imports)]
use std::sync::atomic::{AtomicBool, Ordering::{Release, Acquire, Relaxed}};
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    // if LOCKED.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
    if LOCKED.swap(true, Acquire) == false { // 2nd variant
        unsafe { DATA.push('!') };
        LOCKED.store(false, Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..1000 {
            s.spawn(f);
        }
    });

    println!("Done. DATA: {}", unsafe { DATA.as_str() });
    println!("DATA.len: {}", unsafe { DATA.len() });
}
