use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut average_time = Duration::from_secs(0);
    let mut max_time = Duration::from_secs(0);
    const RUNS: usize = 200;
    let fun = allocate_new_id_1;

    for _ in 0..RUNS {
        let start = Instant::now();
        let new_id = fun();
        let elapsed = start.elapsed();
        if elapsed > max_time {
            max_time = elapsed;
        }
        average_time += elapsed;
        println!("new id: {}, elapsed: {:?}", new_id, elapsed);
    }
    average_time /= RUNS as u32;
    println!("average: {:?}, peak: {:?}", average_time, max_time);
}

fn allocate_new_id_1() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

// This version is problematic.
fn allocate_new_id_2() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    assert!(id < 1000, "too many IDs!");
    id
}

fn allocate_new_id_3() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    if id >= 1000 {
        NEXT_ID.fetch_sub(1, Ordering::Relaxed);
        panic!("too many IDs!");
    }
    id
}
