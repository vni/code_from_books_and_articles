fn main() {
    let numbers = vec![1, 2, 3];

    std::thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    }).join().unwrap();
}
