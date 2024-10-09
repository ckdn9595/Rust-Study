use std::io::{self, BufRead};

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 첫 번째 라인에서 총 개수를 읽습니다.
    let mut total: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut count = 0;

    for line in lines {
        match line.unwrap().trim().parse::<usize>() {
            Ok(v) => {
                total -= v;
                count += 1;

                // 모든 숫자를 처리했다면 루프를 종료합니다.
                if count >= 9 {
                    break;
                }
            }
            Err(_) => {
                println!("Invalid input. Finishing...");
                break;
            }
        }
    }

    println!("{}", total);
}
