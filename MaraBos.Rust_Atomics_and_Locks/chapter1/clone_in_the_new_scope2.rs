use std::sync::Arc;
use std::thread;

fn main() {
    let a = Arc::new([1, 2, 3]);

    thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });

    dbg!(a);

    thread::sleep(std::time::Duration::from_secs(1));
}
