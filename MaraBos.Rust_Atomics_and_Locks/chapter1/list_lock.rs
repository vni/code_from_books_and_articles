use std::sync::Mutex;

fn main() {
    let list: Mutex<Vec<i32>> = Mutex::new(vec![1, 2, 3, 4]);
    list.lock().unwrap().push(5);
    println!("list: {:?}", list.lock().unwrap());
}
