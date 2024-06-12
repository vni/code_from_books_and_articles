use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| {
        while !STOP.load(Ordering::Relaxed) {
            some_work();
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }

    STOP.store(true, Ordering::Relaxed);
    background_thread.join().unwrap();
}

fn some_work() {
    // some have duty staff
    thread::sleep(Duration::from_millis(400));
    println!("... still working ...");
}
