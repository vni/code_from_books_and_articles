use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::thread;
use std::time::Duration;

static OOPS_COUNTER: AtomicU32 = AtomicU32::new(0);

fn main() {
    let counter = &AtomicU32::new(0);

    thread::scope(|s| {
        for _t in 0..16 {
            s.spawn(move || {
                for _ in 0..1000 {
                    increment(counter);
                }
            });

        }
    });

    println!("Done! The resulting counter is {:?}", counter);
    println!("oops_counter: {:?}", OOPS_COUNTER.load(Relaxed));
}

fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => {
                OOPS_COUNTER.fetch_add(1, Relaxed);
                thread::sleep(Duration::from_nanos(1));
                // println!("oops");
                current = v;
            }
        }
    }
}
