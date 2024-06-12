use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{
    fence, AtomicUsize,
    Ordering::{Acquire, Relaxed, Release},
};

// ArcData //

struct ArcData<T> {
    /// Number of `Arc`s.
    strong_count: AtomicUsize,
    /// Number of `Weak`s, plus one if there are any `Arc`s.
    weak_count: AtomicUsize,
    /// The data. Dropped if there are only weak pointers left.
    data: UnsafeCell<ManuallyDrop<T>>,
}

// Arc //

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Sync + Send> Send for Arc<T> {}
unsafe impl<T: Sync + Send> Sync for Arc<T> {}

impl<T> Arc<T> {
    fn inner_data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                strong_count: AtomicUsize::new(1),
                weak_count: AtomicUsize::new(1),
                data: UnsafeCell::new(ManuallyDrop::new(data)),
            }))),
        }
    }

    pub fn downgrade(&self) -> Weak<T> {
        let mut n = self.inner_data().strong_count.load(Relaxed);
        loop {
            if n == usize::MAX {
                std::hint::spin_loop();
                n = self.inner_data().weak_count.load(Relaxed);
                continue;
            }
            assert!(n <= usize::MAX / 2);
            if let Err(e) =
                self.inner_data()
                    .weak_count
                    .compare_exchange_weak(n, n + 1, Acquire, Relaxed)
            {
                n = e;
                continue;
            }
            return Weak { ptr: self.ptr };
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        // Acquire matches Weak::drop's Release decrement, to make sure any
        // upgraded pointers are visible in the next strong_count.load.
        if self
            .inner_data()
            .weak_count
            .compare_exchange(1, usize::MAX, Acquire, Relaxed)
            .is_err()
        {
            return None;
        }

        let is_unique = self.inner_data().strong_count.load(Relaxed) == 1;
        // Release matches Acquire increment in `downgrade`, to make sure any
        // changes to the strong_count that come after `downgrade` don't change
        // the is_unique result above.
        self.inner_data().weak_count.store(1, Release);
        if !is_unique {
            return None;
        }

        // Acquire to match Arc::drop's Release decrement, to make sure nothing
        // else is accessing the data.
        fence(Acquire);
        unsafe { Some(&mut *self.inner_data().data.get()) }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.inner_data().strong_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc { ptr: self.ptr }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: Since there's an Arc to the data,
        // the data exists and may be shared.
        unsafe { &*self.inner_data().data.get() }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.inner_data().strong_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);

            // Safety: The data reference counter is zero,
            // so nothing will access the data anymore.
            unsafe {
                ManuallyDrop::drop(&mut *self.inner_data().data.get());
            }

            // Now that there's no `Arc<T>`s left,
            // drop the implicit weak pointer that represented all `Arc<T>`s.
            drop(Weak { ptr: self.ptr });
        }
    }
}

// Weak //

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Sync + Send> Send for Weak<T> {}
unsafe impl<T: Sync + Send> Sync for Weak<T> {}

impl<T> Weak<T> {
    fn inner_data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.inner_data().strong_count.load(Relaxed);
        loop {
            // None if there is no inner data (because it's already dropped)
            if n == 0 {
                return None;
            }
            assert!(n < usize::MAX / 2);
            if let Err(e) =
                self.inner_data()
                    .strong_count
                    .compare_exchange_weak(n, n + 1, Relaxed, Relaxed)
            {
                n = e;
                continue;
            }
            return Some(Arc { ptr: self.ptr });
        }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.inner_data().weak_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Weak { ptr: self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.inner_data().weak_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Relaxed);
            }
        }

        let x = Arc::new(("hello", DetectDrop));
        let y = x.clone();

        let t = std::thread::spawn(move || {
            println!("other thread: x.0: {}", x.0);
            assert_eq!(x.0, "hello");
        });

        println!("main thread: y.0: {}", y.0);
        assert_eq!(y.0, "hello");
        t.join().unwrap();

        println!("1. NUM_DROPS: {}", NUM_DROPS.load(Relaxed));
        assert_eq!(NUM_DROPS.load(Relaxed), 0);

        drop(y);

        println!("2. NUM_DROP: {}", NUM_DROPS.load(Relaxed));
        assert_eq!(NUM_DROPS.load(Relaxed), 1);
    }

    #[test]
    fn test_mutation() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Relaxed);
            }
        }

        let x = Arc::new((1000, DetectDrop));
        let mut y = x.clone();

        let t = std::thread::spawn(move || {
            println!("other thread: x.0: {}", x.0);
            assert_eq!(x.0, 1000);
        });

        println!("main thread: y.0: {}", y.0);
        assert_eq!(y.0, 1000);
        t.join().unwrap();

        y.get_mut().unwrap().0 = 2000;
        println!("after y get_mut() modification: {}", y.0);
        assert_eq!(y.0, 2000);

        println!("1. NUM_DROPS: {}", NUM_DROPS.load(Relaxed));
        assert_eq!(NUM_DROPS.load(Relaxed), 0);

        drop(y);

        println!("2. NUM_DROP: {}", NUM_DROPS.load(Relaxed));
        assert_eq!(NUM_DROPS.load(Relaxed), 1);
    }

    #[test]
    fn test_weak() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Relaxed);
            }
        }

        let arc = Arc::new(("hello", DetectDrop));
        let weak1 = Arc::downgrade(&arc);
        let weak2 = Arc::downgrade(&arc);

        let t = std::thread::spawn(move || {
            let strong1 = weak1.upgrade().unwrap();
            assert_eq!(strong1.0, "hello");
        });
        assert_eq!(arc.0, "hello");
        t.join().unwrap();

        assert_eq!(NUM_DROPS.load(Relaxed), 0);
        assert!(weak2.upgrade().is_some());

        drop(arc);

        assert_eq!(NUM_DROPS.load(Relaxed), 1);
        assert!(weak2.upgrade().is_none());
    }
}

fn main() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Relaxed);
        }
    }

    let x = Arc::new((100, DetectDrop));
    let mut y = x.clone();

    let t = std::thread::spawn(move || {
        println!("other thread: x.0: {}", x.0);
        assert_eq!(x.0, 100);
    });

    println!("main thread: y.0: {}", y.0);
    assert_eq!(y.0, 100);
    t.join().unwrap();

    y.get_mut().unwrap().0 = 200;
    println!("main thread: y.0 after get_mut() modification {}", y.0);

    println!("1. NUM_DROPS: {}", NUM_DROPS.load(Relaxed));
    assert_eq!(NUM_DROPS.load(Relaxed), 0);

    drop(y);

    println!("2. NUM_DROP: {}", NUM_DROPS.load(Relaxed));
    assert_eq!(NUM_DROPS.load(Relaxed), 1);

    println!("done");
}
