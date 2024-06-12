use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    let m = Arc::new(Mutex::new(100));
    thread::scope(|scope| {
        let mut i = 0;
        for _ in 0..10 {
            i += 1;
            scope.spawn({
                let ii = i;
                let m = Arc::clone(&m);

                move || {
                    if m.is_poisoned() {
                        println!("mutex is poisoned, returning...");
                        return;
                    }

                    let _guard = match m.lock() {
                        Ok(guard) => guard,
                        Err(data) => {
                            println!("Poisoned Mutex caught");
                            println!("recovered data: {}", data.get_ref());
                            // e.guard
                            return;
                        }

                    };
                    println!("thread id: {:?}, i: {}",
                             thread::current().id(), i);

                    if ii == 8 {
                        panic!("oy, 5th thread, panicking...");
                    }
                }
            });
        }
    });
}
