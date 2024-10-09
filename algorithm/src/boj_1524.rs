use std::io::{self, BufRead};

fn find_max_value(vec : Vec<usize>) -> usize {
    let mut max = 0;
    for v in vec {
        if max < v {
            max = v;
        }
    }
    max
}

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut t:i8 = lines.next().unwrap().unwrap().parse().unwrap();
    while {t-=1; t} >= 0 {
        lines.next();
        // let values: Vec<usize> = values.split_whitespace()
        //                             .map(|s| {s.parse().unwrap()})
        //                             .collect();
        // let (s,b) = (values[0], values[1]);
        lines.next();
        let s_list = lines.next().unwrap().unwrap();
        let s_list: Vec<usize> = s_list.split_whitespace()
                                .map(|v| {v.parse().unwrap()})
                                .collect();
        let b_list = lines.next().unwrap().unwrap();
        let b_list: Vec<usize> = b_list.split_whitespace()
                                .map(|v| {v.parse().unwrap()})
                                .collect();
        let s = find_max_value(s_list);
        let b = find_max_value(b_list);

        if s < b {
            println!("B");
        }else {
            println!("S");
        }
    }
}