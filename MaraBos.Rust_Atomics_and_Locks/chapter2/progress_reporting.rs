use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Ordering::Relaxed);
            }
        });

        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 { break; }
            println!("Working... {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done");
}

fn process_item(_i: usize) {
    // processing
    thread::sleep(Duration::from_millis(37));
}
