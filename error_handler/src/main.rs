// mod panic;
// use panic::*;
// mod result;
// use result::*;

// #1
// fn main() {
//     // hello();
//     // println!("정상 동작");
//     println!("{}", get_value(3));
//     println!("{}", get_value(100));
// }

// // #2
//fn main() {
//     file_open();
// }

// #3
// fn main() {
//     let name_result = read_username_from_file();
//     let name = name_result.expect("hello.txt 파일에서 이름 값 꺼내오기 실패!");
//     print!("출력된 값 : {}", name);
// }
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents: {}", contents);
    Ok(())
}
