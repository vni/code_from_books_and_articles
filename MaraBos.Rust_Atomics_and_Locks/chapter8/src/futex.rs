// Futex
// https://marabos.nl/atomics/os-primitives.html#futex

#[cfg(not(target_os = "linux"))]
compile_error!("Linux only. Sorry!");

use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::thread;
use std::time::Duration;

pub fn wait(a: &AtomicU32, expected: u32) {
    // Refer to the futex(2) man page for the syscall signature.
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            a as *const AtomicU32,
            libc::FUTEX_WAIT,
            expected,
            std::ptr::null::<libc::timespec>(),
        );
    }
}

pub fn wake_one(a: &AtomicU32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            a as *const AtomicU32,
            libc::FUTEX_WAKE,
            1, // The number of threads to wake up.
        );
    }
}

fn main() {
    let a = AtomicU32::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            thread::sleep(Duration::from_secs(3));
            a.store(1, Relaxed);
            wake_one(&a);
        });

        println!("Waiting...");
        while a.load(Relaxed) == 0 {
            wait(&a, 0);
        }
        println!("Done!");
    })
}
