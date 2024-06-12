use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }

        Guard {
            lock: self,
        }
    }
}

#[derive(Debug)]
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let x = SpinLock::new(Vec::new());

    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });

    let /* mut */ g = x.lock();
    println!("g.as_slice should be [1, 2, 2] or [2, 2, 1], nothing else");
    println!("g.as_slice: {:?}", g.as_slice());
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);

    // std::mem::drop(g);
    // g.push(3);


    let lock = Arc::new(SpinLock::new(0_usize));

    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10_000 {
                *lock.lock() += 1;
            }
        });

        s.spawn(|| {
            for _ in 0..10_000 {
                *lock.lock() += 2;
            }
        });

        s.spawn(|| {
            for _ in 0..10_000 {
                *lock.lock() += 3;
            }
        });

        s.spawn(|| {
            for _ in 0..10_000 {
                *lock.lock() += 4;
            }
        });

        s.spawn(|| {
            for _ in 0..10_000 {
                *lock.lock() += 5;
            }
        });
    });

    println!("The resulting SUM is: {:#?}", lock.lock());
}
