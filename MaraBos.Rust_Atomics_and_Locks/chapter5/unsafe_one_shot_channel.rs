use std::mem::MaybeUninit;
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    /// Safety: Only call this once!
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    /// Safety: Only call this once,
    /// and only after is_ready() returns true!
    pub unsafe fn receive(&self) -> T {
        (*self.message.get()).assume_init_read()
    }
}

fn main() {
    let chan = Arc::new(Channel::new());
    thread::scope(|scope| {
        scope.spawn(|| {
            unsafe { chan.send(100_500) };
        });

        scope.spawn(|| {
            while !chan.is_ready() {
                thread::yield_now();
            }
            let x = unsafe { chan.receive() };
            println!("received value: {x}");
        });

    });
    println!("done");
}
