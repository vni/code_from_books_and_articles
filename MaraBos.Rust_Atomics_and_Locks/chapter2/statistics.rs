use std::thread;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::time::{Duration, Instant};

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    let time_taken = start.elapsed().as_nanos() as u64;
                    num_done.fetch_add(1, Ordering::Relaxed);
                    total_time.fetch_add(time_taken, Ordering::Relaxed);
                    max_time.fetch_max(time_taken, Ordering::Relaxed);
                }
            });
        }

        loop {
            let total_time = Duration::from_nanos(total_time.load(Ordering::Relaxed));
            let max_time = Duration::from_nanos(max_time.load(Ordering::Relaxed));
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 { break; }
            if n == 0 {
                println!("Working... nothing done yet.");
            } else {
                println!("Working... {n}/100 done, {:.3?} average, {:.3?} peak",
                         total_time / n as u32, max_time);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn process_item(item: usize) {
    // processing...
    thread::sleep(Duration::from_millis(item as u64 * 3 + 100));
}
