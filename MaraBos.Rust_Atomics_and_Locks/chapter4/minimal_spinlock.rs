use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Relaxed, Release},
};
use std::sync::Arc;
use std::thread;

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self
            .locked
            .compare_exchange_weak(false, true, Acquire, Relaxed)
            .is_err()
        {
            std::hint::spin_loop();
        }
        // while self.locked.swap(true, Acquire) {
        //     std::hint::spin_loop();
        // }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

fn main() {
    let lock = Arc::new(SpinLock::new());
    static mut SUM: usize = 0;

    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10_000 {
                lock.lock();
                unsafe { SUM += 1; }
                lock.unlock();
            }
        });

        s.spawn(|| {
            for _ in 0..10_000 {
                lock.lock();
                unsafe { SUM += 2; }
                lock.unlock();
            }
        });

        s.spawn(|| {
            for _ in 0..10_000 {
                lock.lock();
                unsafe { SUM += 3; }
                lock.unlock();
            }
        });

        s.spawn(|| {
            for _ in 0..10_000 {
                lock.lock();
                unsafe { SUM += 4; }
                lock.unlock();
            }
        });

        s.spawn(|| {
            for _ in 0..10_000 {
                lock.lock();
                unsafe { SUM += 5; }
                lock.unlock();
            }
        });
    });

    println!("The resulting SUM is: {}", unsafe { SUM });
}
