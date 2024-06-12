use std::thread;
use std::sync::atomic::{AtomicI32, Ordering::Relaxed};

static X: AtomicI32 = AtomicI32::new(0);

#[allow(dead_code)]
fn a() {
    X.fetch_add(5, Relaxed);
    thread::sleep(std::time::Duration::from_nanos(2));
    X.fetch_add(10, Relaxed);
}

fn a1() {
    X.fetch_add(5, Relaxed);
}

fn a2() {
    X.fetch_add(10, Relaxed);
}

fn b() {
    let a = X.load(Relaxed);
    thread::sleep(std::time::Duration::from_nanos(1));
    let b = X.load(Relaxed);
    thread::sleep(std::time::Duration::from_nanos(1));
    let c = X.load(Relaxed);
    thread::sleep(std::time::Duration::from_nanos(1));
    let d = X.load(Relaxed);
    println!("{a} {b} {c} {d}");
}

fn main() {
    thread::scope(|s| {
        s.spawn(b);
        s.spawn(a2);
        s.spawn(a1);
    });
    println!("done");
}
