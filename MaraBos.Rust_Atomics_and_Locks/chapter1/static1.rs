static X: [i32; 3] = [1, 2, 3];

fn main() {
    std::thread::spawn(|| dbg!(&X));
    std::thread::spawn(|| dbg!(&X));
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("done");
}
