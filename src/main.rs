fn main() {
    for n in 1..=30 {
        if is_multiple_of_n(n, 3) && is_multiple_of_n(n, 5) {
            println!("forbar");
        }
        if is_multiple_of_n(n, 3) {
            println!("foo");
        } else if is_multiple_of_n(n, 5) {
            println!("bar");
        } else {
            println!("{}", n);
        }
    }
}

fn is_multiple_of_n(value: usize, n: usize) -> bool {
    if value % n == 0 {
        true
    } else {
        false
    }
}


