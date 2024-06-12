use std::sync::{RwLock, Arc};
use std::thread;

fn main() {
    let lock = Arc::new(RwLock::new(0));

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn({
                let l = Arc::clone(&lock);

                move || {
                    println!("thread_id: {:?}", thread::current().id());
                    let r = l.read().unwrap();
                    println!("read: {}", r);
                    drop(r);

                    let mut writer = l.write().unwrap();
                    *writer += 1;
                    println!("writer: {}", writer);
                }
            });
        }
    });
}
