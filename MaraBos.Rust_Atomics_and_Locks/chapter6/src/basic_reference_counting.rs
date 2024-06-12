use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{
    fence, AtomicUsize,
    Ordering::{Acquire, Relaxed, Release},
};

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // TODO: Memory ordering.
        if self.data().ref_count.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

#[test]
fn test() {
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

fn main() {
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

    println!("done");
}
