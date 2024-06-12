fn main() {
    let list = std::sync::Mutex::new(vec![1, 2, 3, 4, 5]);

    /*
    if let Some(item) = list.lock().unwrap().pop() {
        // process_item(item);
        println!("process item: {}", item);
    }
    */
    let item = list.lock().unwrap().pop();
    if let Some(item) = item {
        process_item(item);
        println!("process item: {}", item);
    }

    if list.lock().unwrap().pop() == Some(4) {
        println!("popped 4 from the list");
    }
}

#[allow(unused)]
fn process_item(item: i32) {
    println!("processing item {}", item);
}
