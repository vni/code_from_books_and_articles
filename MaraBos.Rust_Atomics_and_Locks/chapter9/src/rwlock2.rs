// Avoiding Busy-Looping Writers
// https://marabos.nl/atomics/building-locks.html#avoiding-busy-looping-writers

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
    /// Incremented to wake up writers.
    writer_wake_counter: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for RWLock<T> where T: Send + Sync {}

impl<T> RWLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            writer_wake_counter: AtomicU32::new(0),
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
        while self
            .state
            .compare_exchange(0, u32::MAX, Acquire, Relaxed)
            .is_err()
        {
            let w = self.writer_wake_counter.load(Acquire);
            if self.state.load(Relaxed) != 0 {
                // Wait if the RWLock is still locked, but only if
                // there have been no wake signals since we checked.
                wait(&self.writer_wake_counter, w);
            }
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
            self.rwlock.writer_wake_counter.fetch_add(1, Release);
            wake_one(&self.rwlock.writer_wake_counter);
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
        self.rwlock.writer_wake_counter.fetch_add(1, Release);
        // Wake up all waiting readers and writers.
        wake_one(&self.rwlock.writer_wake_counter);
        wake_all(&self.rwlock.state);
    }
}

// main //

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
