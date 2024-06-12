struct X {
    p: *mut i32,
}

unsafe impl Send for X {}
unsafe impl Sync for X {}
