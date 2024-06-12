use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::collections::VecDeque;

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one(); // .notify_all()
    }

    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }
            b = self.item_ready.wait(b).unwrap();
        }
    }
}

fn main() {
    let chan = Arc::new(Channel::new());

    thread::scope(|s| {
        s.spawn(|| {
            for i in 1..100 {
                chan.send(i);
                thread::sleep(std::time::Duration::from_micros(1));
            }
            chan.send(0);
        });

        s.spawn(|| {
            for i in 1001..1100 {
                chan.send(i);
                thread::sleep(std::time::Duration::from_micros(1));
            }
            chan.send(0);

        });

        s.spawn(|| {
            let mut zeros = 0;
            loop {
                let x = chan.receive();
                println!("received: {x}");
                if x == 0 {
                    zeros += 1;
                }
                if zeros == 2 {
                    break;
                }
            }
        });
    });

    println!("done");
}
