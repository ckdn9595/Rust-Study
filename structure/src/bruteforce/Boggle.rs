use std::io::{self, BufRead};

pub fn main(){
    // let mut input = String::new();
    // stdin().read_to_string(&mut input).unwrap();
    // let mut input = input.split_ascii_whitespace();
    
    // let n = input.next().unwrap().parse::<usize>().unwrap();
    // let line = input.next().unwrap();
    // println!("{}, {}", n, line);

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut matrix: Vec<Vec<String>> = Vec::new();

    // 첫 줄에서 n을 읽습니다.
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // n만큼의 줄을 읽어 2차원 벡터에 저장합니다.
    for _ in 0..5 {
        let line = lines.next().unwrap().unwrap();
        matrix.push(vec![line]);
    }

    // 저장된 2차원 벡터를 출력합니다.
    println!("{:?}", matrix);
}