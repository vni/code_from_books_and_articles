fn factorial(n: u64) -> u64 {
    (1..n).product()
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn my_function() {
    println!("hello world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
