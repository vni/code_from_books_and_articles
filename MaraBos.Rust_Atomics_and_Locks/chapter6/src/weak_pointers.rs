use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{
    fence, AtomicUsize,
    Ordering::{Acquire, Relaxed, Release},
};

struct ArcData<T> {
    strong_count: AtomicUsize,
    weak_count: AtomicUsize, // actually, this is strong and weak counter
    data: UnsafeCell<Option<T>>,
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().strong_count.load(Relaxed);
        loop {
            if n == 0 {
                return None;
            }
            assert!(n < usize::MAX / 2);
            if let Err(e) =
                self.data()
                    .strong_count
                    .compare_exchange_weak(n, n + 1, Relaxed, Relaxed)
            {
                n = e;
                continue;
            }
            return Some(Arc { weak: self.clone() });
        }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.data().weak_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Weak { ptr: self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self.data().weak_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

pub struct Arc<T> {
    weak: Weak<T>,
}

unsafe impl<T: Send + Sync> Send for Weak<T> {}
unsafe impl<T: Send + Sync> Sync for Weak<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(ArcData {
                    strong_count: AtomicUsize::new(1),
                    weak_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                }))),
            },
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.weak.data().weak_count.load(Relaxed) == 1 {
            fence(Acquire);
            // Safety: Nothing else can access the data, since
            // there's only one Arc, to which we have exclusive access,
            // and no Weak pointers.
            let arcdata = unsafe { self.weak.ptr.as_mut() };
            let option = arcdata.data.get_mut();

            // We know the data is still available since we
            // have an Arc to it, so this won't panic.

            // FIXME:
            // let data = option.as_mut().unwrap();
            // Some(data)

            option.as_mut()
        } else {
            None
        }
    }

    pub fn downgrade(&self) -> Weak<T> {
        self.weak.clone()
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let ptr = self.weak.data().data.get();
        // Safety: Since there's an Arc to the data,
        // the data exists and may be shared.
        unsafe { (*ptr).as_ref().unwrap() }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let weak = self.weak.clone();
        if weak.data().strong_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc { weak }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.weak.data().strong_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            let ptr = self.weak.data().data.get();
            // Safety: The data reference counter is zero,
            // so nothing will access it.
            unsafe {
                (*ptr) = None;
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
