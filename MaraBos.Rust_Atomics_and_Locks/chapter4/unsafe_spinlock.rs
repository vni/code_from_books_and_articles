use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub struct UnsafeSpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for UnsafeSpinLock<T> where T: Send {}

impl<T> UnsafeSpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock<'a>(&self) -> &'a mut T {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
        unsafe { &mut *self.value.get() }
    }

    pub unsafe fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let lock1 = Arc::new(UnsafeSpinLock::new(0_usize));
    let lock2 = Arc::clone(&lock1);
    let lock3 = Arc::clone(&lock1);
    let lock4 = Arc::clone(&lock1);
    let lock5 = Arc::clone(&lock1);

    thread::scope(|s| {
        s.spawn(move || {
            for _ in 0..10_000 {
                let inner = lock1.lock();
                *inner += 1;
                unsafe { lock1.unlock() };
            }
        });

        s.spawn(move || {
            for _ in 0..10_000 {
                *lock2.lock() += 2;
                unsafe { lock2.unlock() };
            }
        });

        s.spawn(move || {
            for _ in 0..10_000 {
                *lock3.lock() += 3;
                unsafe { lock3.unlock() };
            }
        });

        s.spawn(|| {
            for _ in 0..10_000 {
                *lock4.lock() += 4;
                unsafe { lock4.unlock() };
            }
        });

        s.spawn(|| {
            for _ in 0..10_000 {
                *lock5.lock() += 5;
                unsafe { lock5.unlock() };
            }
        });
    });

    let v = lock4.lock();
    unsafe { lock4.unlock() };
    println!("The resulting SUM is: {}", v);
}
