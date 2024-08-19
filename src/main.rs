fn main() {
    for n in 1..=30 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("forbar");
        }
        if n % 3 == 0 {
            println!("foo");
        } else if n % 5 == 0 {
            println!("bar");
        } else {
            println!("{}", n);
        }
    }
}
