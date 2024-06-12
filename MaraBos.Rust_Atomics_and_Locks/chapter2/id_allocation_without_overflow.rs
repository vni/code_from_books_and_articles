use std::thread;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

static OOPS_COUNTER: AtomicU32 = AtomicU32::new(0);

fn main() {
    thread::scope(|s| {
        for _ in 0..16 {
            s.spawn(|| {
                for _ in 0..1000 {
                    println!("thread: {:?}, allocate_new_id: {}",
                             thread::current().id(), allocate_new_id_2());
                }
            });
        }
    });
    println!("oops counter: {:?}", OOPS_COUNTER.load(Relaxed));
}

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Relaxed);
    loop {
        assert!(id < 1_000_000, "too many IDs!");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(v) => {
                OOPS_COUNTER.fetch_add(1, Relaxed);
                id = v;
            }
        }
    }
}

fn allocate_new_id_2() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_update(Relaxed, Relaxed, |n| n.checked_add(1)).expect("too many IDs!")
}
