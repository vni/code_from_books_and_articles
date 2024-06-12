use std::thread;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

fn main() {
    thread::scope(|s| {
        for _ in 0..16 {
            s.spawn(|| {
                let key = get_key();
                println!("key: {:?}", key);
            });
        }
    });
    println!("done");
}

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Relaxed);
    if key == 0 {
        let new_key = generate_random_key();
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => {
                println!("oops, key initialization clash");
                k
            }
        }
    } else {
        key
    }
}

fn generate_random_key() -> u64 {
    314159265383 // random :)
}
