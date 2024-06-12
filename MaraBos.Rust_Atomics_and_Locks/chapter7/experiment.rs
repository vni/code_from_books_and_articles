use std::sync::atomic::{compiler_fence, AtomicBool, AtomicUsize, Ordering::*};

fn main() {
    let locked = AtomicBool::new(false);
    let counter = AtomicUsize::new(0);

    std::thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| for _ in 0..1_000_000 {
                while locked.swap(true, Relaxed) {}
                compiler_fence(Acquire);

                let old = counter.load(Relaxed);
                let new = old + 1;
                counter.store(new, Relaxed);

                compiler_fence(Release);
                locked.store(false, Relaxed);
            });
        }
    });

    println!("{}", counter.into_inner());
}
