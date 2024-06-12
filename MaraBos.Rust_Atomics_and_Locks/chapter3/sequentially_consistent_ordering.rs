use std::thread;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
    let b = thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe { S.push('B') };
        }
    });

    let a = thread::spawn(|| {
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe { S.push('A') };
        }
    });

    a.join().unwrap();
    b.join().unwrap();

    println!("S: {}", unsafe { /* AsRef::<str>::as_ref(&S) */ S.as_str() });
    println!("S.len(): {}", unsafe{S.len()});
}
