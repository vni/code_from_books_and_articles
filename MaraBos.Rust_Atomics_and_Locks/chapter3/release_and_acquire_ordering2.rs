use std::sync::atomic::{AtomicBool, Ordering::{Release, Acquire}};
use std::time::Duration;
use std::thread;

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    let t = thread::spawn(|| {
        unsafe { DATA = 123 };
        READY.store(true, Release);
    });

    while !READY.load(Acquire) {
        thread::sleep(Duration::from_nanos(1));
        println!("waiting...");
    }
    
    println!("DATA: {}", unsafe { DATA });
    t.join().unwrap();
}
