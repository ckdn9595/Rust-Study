use std::io::{self, BufRead};

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for i in 1..=n {
        let stars = "*".repeat(i);
        println!("{:>width$}", stars, width = n);
    }

    for i in (1..n).rev() {
        let stars = "*".repeat(i);
        println!("{:>width$}", stars, width = n);
    }
}