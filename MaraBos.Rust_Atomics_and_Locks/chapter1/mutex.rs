use std::sync::Mutex;

fn main() {
    let n = Mutex::new(0);
    std::thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }

                drop(guard);
                std::thread::sleep(std::time::Duration::from_secs(1));
            });
        }
    });

    let inner = n.into_inner().unwrap();
    println!("inner: {}", inner);
    assert_eq!(inner, 1000);
}
