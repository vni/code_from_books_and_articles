// Avoiding Writer Starvation
// https://marabos.nl/atomics/building-locks.html#avoiding-writer-starvation

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
    /// The number of read lockes times two, plus on if there's a writer waiting.
    /// u32::MAX if write locked.
    ///
    /// This means that readers may acquire the lock when
    /// the state is even, but need to block when odd.
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
            if s % 2 == 0 {
                // Even
                assert!(s < u32::MAX - 2, "too many readers");
                match self.state.compare_exchange_weak(s, s + 2, Acquire, Relaxed) {
                    Ok(_) => return ReadGuard { rwlock: self },
                    Err(new_s) => s = new_s,
                }
            }

            if s % 2 == 1 {
                // Odd.
                wait(&self.state, s);
                s = self.state.load(Relaxed);
            }
        }
    }

    pub fn write(&self) -> WriteGuard<T> {
        let mut s = self.state.load(Relaxed);
        loop {
            // Try lock if unlocked.
            if s <= 1 {
                match self.state.compare_exchange(s, u32::MAX, Acquire, Relaxed) {
                    Ok(_) => return WriteGuard { rwlock: self },
                    Err(new_s) => {
                        s = new_s;
                        continue;
                    }
                }
            }

            // Block new readers, by making sure the state is odd.
            if s % 2 == 0 {
                match self.state.compare_exchange(s, s + 1, Acquire, Relaxed) {
                    Ok(_) => {}
                    Err(new_s) => {
                        s = new_s;
                        continue;
                    }
                }
            }

            // Wait, if it's still locked
            let w = self.writer_wake_counter.load(Acquire);
            s = self.state.load(Relaxed);
            // If there are readers
            if s >= 2 {
                wait(&self.writer_wake_counter, w);
                s = self.state.load(Relaxed);
            }
        }
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
        // Decrement the state by 2 to remove one read-lock.
        if self.rwlock.state.fetch_sub(2, Release) == 3 {
            // If we decrement from 3 to 1, that means
            // the RWLock is now unlocked _and_ there is
            // a waiting writer, which we wake up.
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
