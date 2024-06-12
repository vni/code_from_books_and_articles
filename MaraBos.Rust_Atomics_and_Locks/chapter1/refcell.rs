/*
Unlike a regular Cell, a std::cell::RefCell does allow you to borrow its contents, at a small runtime cost. A RefCell<T> does not only hold a T, but also holds a counter that keeps track of any outstanding borrows. If you try to borrow it while it is already mutably borrowed (or vice-versa), it will panic, which avoids undefined behavior. Just like a Cell, a RefCell can only be used within a single thread.
*/

use std::cell::RefCell;

fn main() {
    let v = RefCell::new(vec![1, 2, 3, 4]);
    println!("v before: {:?}", v.borrow());
    f(&v);
    println!("v after: {:?}", v.borrow());
}

fn f(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1); // We can modify the Vec directly
}
