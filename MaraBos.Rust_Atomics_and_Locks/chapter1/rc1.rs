use std::rc::Rc;

fn main() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());
    println!("a.as_ptr: {:p}", a.as_ptr());
    println!("b.as_ptr: {:p}", b.as_ptr());

    // std::thread::spawn(move || dbg!(b));
}
