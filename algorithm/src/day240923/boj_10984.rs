use std::io::{self, BufRead};

fn parse_num<T: std::str::FromStr>(lines: &mut dyn Iterator<Item = Result<String, io::Error>>) -> Option<T> {
    lines.next()?.ok()?.trim().parse().ok()
}

fn split_nums(line: &str) -> Vec<String> {
    line.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

struct Score {
    count: i32,
    average: f32,
}

impl Score {
    fn new(count: i32, average: f32) -> Self {
        Score { count, average }
    }

    fn print_score(&self) {
        println!("{} {:.2}", self.count, self.average);
    }
}

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut t: usize = parse_num(&mut lines).unwrap_or(0);
    let mut scores = Vec::new();

    while t > 0 {
        if let Some(cnt) = parse_num::<usize>(&mut lines) {
            let mut count:i32 = 0;
            let mut sum:f32 = 0.0;
            for i in 0..cnt {
                if let Some(line) = lines.next().and_then(Result::ok) {
                    let values = split_nums(&line);
                    let size = values[0].parse::<i32>().unwrap();
                    count += size;
                    sum  += values[1].parse::<f32>().unwrap() * (size as f32);
                }
            }
            scores.push(Score::new(count,  sum / count as f32));
        }
        t -= 1;
    }

    for score in scores {
        score.print_score();
    }

}