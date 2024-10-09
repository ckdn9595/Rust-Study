use std::io::{self, BufRead};

pub fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut max_num = 0;
    let mut max_idx = -1;
    for idx in 1..10 {
        let v = lines.next().unwrap().unwrap();
        let v:i32 = v.parse().unwrap();
        if max_num < v {
            max_num = v;
            max_idx = idx;
        }
    }
    println!("{}", max_num);
    println!("{}", max_idx);
}