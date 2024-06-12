fn main() {
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    std::thread::spawn(move || dbg!(x));
    std::thread::spawn(move || dbg!(x));

    std::thread::sleep(std::time::Duration::from_secs(1));
}
