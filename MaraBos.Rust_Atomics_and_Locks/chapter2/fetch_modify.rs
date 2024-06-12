/*
impl AtomicI32 {
    pub fn fetch_add(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_sub(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_or(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_and(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_nand(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_xor(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_max(&self, v: i32, ordering: Ordering) -> i32;
    pub fn fetch_min(&self, v: i32, ordering: Ordering) -> i32;
    pub fn swap(&self, v: i32, ordering: Ordering) -> i32; // "fetch_store"
}
*/

use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let a = AtomicI32::new(100);
    let b = a.fetch_add(23, Ordering::Relaxed);
    let c = a.load(Ordering::Relaxed);

    assert_eq!(b, 100);
    assert_eq!(c, 123);

    println!("a: {a:?}, b: {b:?}, c: {c:?}");
}
