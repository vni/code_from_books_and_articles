use std::cell::UnsafeCell;
use std::io::Write;
use std::mem::MaybeUninit;
use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, /*Relaxed,*/ Release},
};
use std::thread;
use std::time::Duration;

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

    // pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
    pub fn split(&mut self) -> (Sender<T>, Receiver<T>) {
        *self = Self::new();
        (Sender { channel: self }, Receiver { channel: self })
    }
}

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Release);
    }
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Receiver<'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Acquire)
        // self.channel.ready.load(Relaxed)
    }

    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Acquire) {
            panic!("no message available!");
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { (*self.message.get()).assume_init_drop() }
        }
    }
}

fn main() {
    let mut channel = Channel::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        // let t = thread::current();
        s.spawn(move || {
            thread::sleep(Duration::from_secs(2));
            sender.send("hello world!");
            // t.unpark();
        });

        while !receiver.is_ready() {
            print!(".");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(400));
            // thread::park();
        }
        println!();

        let msg = receiver.receive();
        println!("received msg: {msg}");
        assert_eq!(msg, "hello world!");
    });
}
