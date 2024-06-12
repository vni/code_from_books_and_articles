// -O --target=aarch64-unknown-linux-musl

pub fn add_ten(num: &mut i32) {
    *num += 10;
}

fn main() {
    let mut v = 20;
    add_ten(&mut v);
    println!("v: {v}");
}
