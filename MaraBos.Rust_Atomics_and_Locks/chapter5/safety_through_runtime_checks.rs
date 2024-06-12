use std::cell::UnsafeCell;
use std::io::Write;
use std::mem::MaybeUninit;
use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Relaxed, Release},
};
use std::thread;
use std::time::Duration;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    in_use: AtomicBool,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            in_use: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        }
    }

    /// Panics when trying to send more than one message.
    pub fn send(&self, message: T) {
        if self.in_use.swap(true, Relaxed) {
            panic!("can't send more than one message!");
        }

        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    /// Panics if no message is available yet.
    ///
    /// Tip: Use `is_ready` to check first.
    ///
    /// Safety: Only call this once!
    /// (but it possible to call it twice, even thou it's UB)
    pub unsafe fn receive_bad(&self) -> T {
        if !self.ready.load(Acquire) {
            panic!("no message available!");
        }

        (*self.message.get()).assume_init_read()
    }

    /// Panics if no message is available yet,
    /// or if the message already consumed.
    ///
    /// Tip: use `is_ready` to check first.
    pub fn receive(&self) -> T {
        if !self.ready.swap(false, Acquire) {
            panic!("no message available");
        }

        // Safety: We've just checked (and reset) the ready flag.
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

fn main() {
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            thread::sleep(Duration::from_secs(1));
            channel.send("hello world!");
            // channel.send("hello world!");
            t.unpark();
        });

        while !channel.is_ready() {
            print!(".");
            std::io::stdout().flush().unwrap();
            thread::park();
        }
        let msg = channel.receive();
        // let msg2 = unsafe { channel.receive() };
        assert_eq!(msg, "hello world!");
        println!("msg: {msg}");
        // println!("msg2: {msg2}");
    });
    println!("done");
}
