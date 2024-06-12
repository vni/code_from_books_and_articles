use std::iter::FromIterator;

fn main() {
    let numbers = Vec::from_iter(0..=1000);
    // let numbers = Vec::new(); // will panic!()

    let t = std::thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();

        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");
}
