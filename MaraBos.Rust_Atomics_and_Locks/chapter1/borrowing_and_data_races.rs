fn main() {
    let a = 10;
    let mut b = 10;

    f(&a, &mut b);
    println!("done");
}

fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;

    if before != after {
        x(); // never happens
    }
}

fn x() {
    panic!("Oops");
}
