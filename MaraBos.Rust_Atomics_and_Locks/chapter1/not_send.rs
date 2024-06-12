fn main() {
    // let a = std::rc::Rc::new(123); // Rc *is not* Send
    let a = std::cell::Cell::new(123);
    std::thread::spawn(move || dbg!(a)).join().unwrap();
}
