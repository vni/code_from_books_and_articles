// Reader-Writer Lock
// https://marabos.nl/atomics/building-locks.html#reader-writer-lock

use atomic_wait::{wait, wake_all, wake_one};
// use `core` instead of `std` to be able run this code in `no_std` envirement.
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{
    AtomicU32,
    Ordering::{Acquire, Relaxed, Release},
};
use std::thread;

// RWLock //

pub struct RWLock<T> {
    /// The number of readers, or u32::MAX if write-locked.
    state: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for RWLock<T> where T: Send + Sync {}

impl<T> RWLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    pub fn read(&self) -> ReadGuard<T> {
        let mut s = self.state.load(Relaxed);
        loop {
            if s < u32::MAX {
                assert!(s < u32::MAX - 1, "too many readers");
                match self.state.compare_exchange_weak(s, s + 1, Acquire, Relaxed) {
                    Ok(_) => return ReadGuard { rwlock: self },
                    Err(e) => s = e,
                }
            }
            if s == u32::MAX {
                wait(&self.state, u32::MAX);
                s = self.state.load(Relaxed);
            }
        }
    }

    pub fn write(&self) -> WriteGuard<T> {
        while let Err(s) = self.state.compare_exchange(0, u32::MAX, Acquire, Relaxed) {
            // Wait while already locked.
            wait(&self.state, s);
        }

        WriteGuard { rwlock: self }
    }
}

// ReadGuard //

pub struct ReadGuard<'a, T> {
    rwlock: &'a RWLock<T>,
}

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> Drop for ReadGuard<'_, T> {
    fn drop(&mut self) {
        if self.rwlock.state.fetch_sub(1, Release) == 1 {
            // Wake up a waiting writer, if any.
            wake_one(&self.rwlock.state);
        }
    }
}

// WriteGuard //

pub struct WriteGuard<'a, T> {
    rwlock: &'a RWLock<T>,
}

impl<T> Deref for WriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> DerefMut for WriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.rwlock.value.get() }
    }
}

impl<T> Drop for WriteGuard<'_, T> {
    fn drop(&mut self) {
        self.rwlock.state.store(0, Release);
        // Wake up all waiting readers and writers.
        wake_all(&self.rwlock.state);
    }
}

fn main() {
    let rwlock = RWLock::new(0);
    static mut SUM: usize = 0;

    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..1000 {
                unsafe { SUM += *rwlock.read() };
            }
        });

        s.spawn(|| {
            for _ in 0..1000 {
                unsafe { SUM += *rwlock.read() };
            }
        });

        s.spawn(|| {
            for _ in 0..1000 {
                *rwlock.write() += 1;
            }
        });
    });

    println!("The sum is: {}", unsafe { SUM });

    println!("done");
}
