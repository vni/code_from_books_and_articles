use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let queue = Arc::new(Mutex::new(VecDeque::new()));

    let queue1 = Arc::clone(&queue);
    let queue2 = Arc::clone(&queue);
    let queue3 = Arc::clone(&queue);
    thread::scope(|s| {
        let consumer = Arc::new(s.spawn(|| loop {
            let item = queue1.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park(); // park the current thread
            }
        }));

        s.spawn({
            let consumer = Arc::clone(&consumer);

            move || {
                for i in 0.. {
                    queue2.lock().unwrap().push_back(i);
                    consumer.thread().unpark(); // wake up the slept and waiting thread
                    thread::sleep(Duration::from_secs(1));
                }
            }
        });

        s.spawn({
            let consumer = Arc::clone(&consumer);

            move || {
                for i in 0.. {
                    queue3.lock().unwrap().push_back(i);
                    consumer.thread().unpark(); // wake up the slept and waiting thread
                    thread::sleep(Duration::from_millis(300));
                }
            }
        });
    });
}
