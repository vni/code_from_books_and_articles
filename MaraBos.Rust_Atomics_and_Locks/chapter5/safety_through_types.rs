use std::cell::UnsafeCell;
use std::io::Write;
use std::mem::MaybeUninit;
use std::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Relaxed, Release},
};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

// unsafe impl<T> Sync for Channel<T> where T: Send {}
unsafe impl<T: Send> Sync for Channel<T> {}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        unsafe { self.message.get_mut().assume_init_drop() }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });

    (Sender { channel: a.clone() }, Receiver { channel: a })
}

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Release);
    }
}

pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }

    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Acquire) {
            panic!("no message available!");
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

fn main() {
    thread::scope(|s| {
        let (sender, receiver) = channel();
        // let t = thread::current();
        s.spawn(move || {
            thread::sleep(Duration::from_secs(2));
            sender.send("hello world!");
            // t.unpark();
        });

        while !receiver.is_ready() {
            print!(".");
            std::io::stdout().flush().unwrap();
            // thread::park();
            thread::sleep(Duration::from_millis(400));
        }
        println!();

        let msg = receiver.receive();
        println!("received msg: {msg}");
        assert_eq!(msg, "hello world!");
    });
}
