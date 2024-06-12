use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};

const TOTAL_ITEMS: usize = 1_000;

fn main() {
    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..TOTAL_ITEMS {
                process_item(i);
                num_done.store(i + 1, Ordering::Relaxed);
                main_thread.unpark();
            }
        });

        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == TOTAL_ITEMS { break; }
            println!("Working... {n}/{TOTAL_ITEMS} done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn process_item(_i: usize) {
    // processing ...
    thread::sleep(Duration::from_millis(27));
}
