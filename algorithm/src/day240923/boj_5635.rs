use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Student(String, u8, u8, u16);  // year를 u16으로 변경

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut t: u8 = lines.next().unwrap().unwrap().parse().unwrap();

    let info = lines.next().unwrap().unwrap();
    let values: Vec<String> = info.split_whitespace().map(|s| {s.to_string()}).collect();
    let name = values[0].to_string();
    let day: u8 = values[1].parse().unwrap();
    let month: u8 = values[2].parse().unwrap();
    let year: u16 = values[3].parse().unwrap();  // u16으로 변경
    let mut last_student = Student(name.clone(), day, month, year);
    let mut first_student = Student(name.clone(), day, month, year);
    t -= 1;

    while t > 0 {
        let values: Vec<String> = lines.next().unwrap().unwrap().split_whitespace().map(|s| {s.to_string()}).collect();
        let name = values.get(0).unwrap();
        let day :u8 = values[1].parse().unwrap();
        let month = values[2].parse().unwrap();
        let year = values[3].parse().unwrap();

        if !compare_birth(&last_student, day, month, year) {
            last_student = Student(name.to_string(), day, month, year);
        }

        if compare_birth(&first_student, day, month, year) {
            first_student = Student(name.to_string(), day, month, year);
        }
        
        t -= 1;
    }
    println!("{}", last_student.0);
    println!("{}", first_student.0);
}

// 기존이 크면 true, 새로운게 크면 false  * 크다는 뜻은 어리다를 의미
fn compare_birth(s: &Student, d: u8, m: u8, y: u16) -> bool {
    if s.3 > y {
        return true
    } else if s.3 < y {
        return false
    }

    if s.2 > m {
        return true
    } else if s.2 < m {
        return false
    }

    if s.1 > d {
        return true
    } else if s.1 < d {
        return false
    }

    true
}