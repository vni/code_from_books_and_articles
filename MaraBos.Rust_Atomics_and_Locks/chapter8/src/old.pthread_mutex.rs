// Unfinished

/*

// use libc::{pthread_mutex_t, *};
use std::cell::UnsafeCell;

// Mutex //

pub struct Mutex<T> {
    // m: libc::pthread_mutex_t,
    // m: UnsafeCell<libc::pthread_mutex_t>,
    m: Box<UnsafeCell<libc::pthread_mutex_t>>,
    value: T,
}

impl<T> Mutex<T> {
    // fn lock(&self) -> MutexGuard {
    fn new() {
        // init
        libc::pthread_mutex_init(self.m.get());
    }
    fn lock(&self) {
        unsafe { libc::pthread_mutex_lock(self.m.get()) };
        // MutexGuard {}
    }

    fn unlock(&self) {
        unsafe { libc::pthread_mutex_unlock(self.m.get()) };
    }
}

// MutexGuard //

struct MutexGuard<T> {
    m: &Mutex<T>,
}

impl<T> Drop for MutexGuard<T> {
    fn drop(&mut self) {
        // what ??
    }
}

impl<T> std::ops::Deref for MutexGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // what ??
    }
}

impl<T> std::ops::DerefMut for MutexGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        // fixme: finish here
    }
}


fn main() {
    /*
    let m = Mutex::new(..);

    let guard = m.lock();
    std::mem::forget(guard); // .. but don't unlock it.
    */
}

*/

struct XMutex {
    m: UnsafeCell<libc::pthread_mutex_t>,
}

unsafe impl Send for XMutex {}
unsafe impl Sync for XMutex {}

impl XMutex {
    pub fn new() -> Self {
        // let mut m = MaybeUninit::<libc::pthread_mutex_t>::uninit();
        // let mut m = libc::PTHREAD_MUTEX_INITIALIZER;
        // unsafe {
        //     if libc::pthread_mutex_init(m.as_mut_ptr(), std::ptr::null()) != 0 {
        //         panic!("Failed to initialize posix mutex");
        //     };
        // }

        Self {
            m: UnsafeCell::new(libc::PTHREAD_MUTEX_INITIALIZER),
        }
    }

    pub fn lock(&mut self) {
        unsafe {
            // libc::pthread_mutex_lock((&mut self.m) as *mut libc::pthread_mutex_t);
            libc::pthread_mutex_lock(self.m.get() as *mut libc::pthread_mutex_t);
        }
    }

    pub fn unlock(&mut self) {
        unsafe {
            // libc::pthread_mutex_unlock((&mut self.m) as *mut libc::pthread_mutex_t);
            libc::pthread_mutex_unlock(self.m.get() as *mut libc::pthread_mutex_t);
        }
    }
}

impl Drop for XMutex {
    fn drop(&mut self) {
        unsafe {
            // libc::pthread_mutex_destroy((&mut self.m) as *mut libc::pthread_mutex_t);
            libc::pthread_mutex_destroy(self.m.get() as *mut libc::pthread_mutex_t);
        }
    }
}

use std::cell::UnsafeCell;
use std::sync::Arc;

fn main() {
    // let mut m1 = Arc::new(XMutex::new());
    let mut m1 = XMutex::new();
    // let m2 = Arc::clone(&m1);
    static mut DATA: [u32; 2] = [0u32, 0u32];

    std::thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..1_000_000 {
                    m1.lock();
                    unsafe {
                        DATA[0] += 1;
                        DATA[1] += 1;
                    }
                    m1.unlock();
                }
            });
        }

        /*
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..1_000_000 {
                    m2.lock();
                    unsafe {
                        DATA[1] += 1;
                        DATA[0] += 1;
                    }
                    m2.unlock();
                }
            });
        }
        */
    });

    unsafe {
        println!("data: [{}, {}]", DATA[0], DATA[1]);
    }
}
