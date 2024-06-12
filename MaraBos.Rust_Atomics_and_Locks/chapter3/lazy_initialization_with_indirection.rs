use std::sync::atomic::{AtomicPtr, Ordering::{Release, Acquire}};
use std::time::{Duration, Instant};
use std::thread;

#[derive(Debug)]
struct Data {
    value: u64,
    timestamp: Instant,
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());
    let mut p = PTR.load(Acquire);
    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(), p, Release, Acquire
        )
        {
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    unsafe { &*p }
}

fn generate_data() -> Data {
    Data {
        value: 314159265,
        timestamp: Instant::now(),
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let data = get_data();
                println!("data: {:?}", data);
            });
        }
    });
}
