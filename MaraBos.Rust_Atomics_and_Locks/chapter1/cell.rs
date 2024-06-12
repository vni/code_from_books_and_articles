// std::cell::Cell  - allows to copy out (if T is Copy)
// or replace it with another value as a whole.
// It can only be used in a single thread.

use std::cell::Cell;

fn main() {
    let a = Cell::new(10);
    let b = Cell::new(20);

    f(&a, &b); // f(&a, &a);
    println!("done");

    let vec = Cell::new(vec![1, 2, 3, 4, 5]);
    f2(&vec);
    let v = vec.take();
    println!("vec: {:?}", v);
}

fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();

    if before != after {
        x(); // might happen
    }
}

fn f2(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take(); // Replace the content of the Cell with an empty Vec
    v2.push(1);
    v.set(v2); // Put the modified Vec beck
}

fn x() {
    panic!("oops");
}
