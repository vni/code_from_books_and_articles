use std::sync::Arc;
use std::thread;
fn main() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    thread::spawn(move || {
        dbg!(b/* .sort() */);
    });

    dbg!(a);

    thread::sleep(std::time::Duration::from_secs(1));
}
