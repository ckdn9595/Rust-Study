use std::io::{self, BufRead};

pub fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut n = lines.next().unwrap().unwrap().parse().unwrap();
    let mut sum = 0;
    loop{
        sum += &n;
        n -= 1;
        if n == 0 {
            break;
        }
    }
    println!("{}", sum);
}