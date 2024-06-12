fn main() {
    let /* mut */ numbers = vec![1, 2, 3];

    std::thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
            // numbers.push(10); // will not compile
        });

        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
            // numbers.push(20); // will not compile
        });
    });

    println!("main: numbers: {numbers:?}");
    println!("end of main");
}
