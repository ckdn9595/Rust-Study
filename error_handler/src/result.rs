use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};


// pub fn file_open() {

//     // #2-1
//     // let greeting_file_result = File::open("hello.txt");
//     //File::open 함수는 Result<std::fs::File, std::io::Error>를 반환
//     // let greeting_file = match greeting_file_result {
//     //     Ok(file) => file,
//     //     Err(error) => panic!("Problem opening the file: {:?}", error),
//     // };
//     // dbg!("{}", greeting_file);
//     // #2-2
//     // let greeting_file = match File::open("hello.txt") {
//     //     Ok(file) => file,
//     //     Err(error) => match error.kind() {
//     //         ErrorKind::NotFound => match File::create("hello.txt") {
//     //             Ok(fc) => fc,
//     //             Err(e) => panic!("Problem creating the file: {:?}", e),
//     //         },
//     //         other_error => {
//     //             panic!("Problem opening the file: {:?}", other_error);
//     //         }
//     //     },
//     // };
//     // fn err_handler(error: std::io::Error) -> std::fs::File {
//     //     if error.kind() == ErrorKind::NotFound {
//     //         File::create("hello.txt").unwrap_or_else(|error| {
//     //             panic!("Problem creating the file: {:?}", error);
//     //         })
//     //     } else {
//     //         panic!("Problem opening the file: {:?}", error);
//     //     }
//     // }
//     // // // #2-3
//     // let greeting_file = File::open("hello.txt").unwrap_or_else(err_handler);
//     // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//     //     if error.kind() == ErrorKind::NotFound {
//     //         File::create("hello.txt").unwrap_or_else(|error| {
//     //             panic!("Problem creating the file: {:?}", error);
//     //         })
//     //     } else {
//     //         panic!("Problem opening the file: {:?}", error);
//     //     }
//     // });

//     // #2-4
//     let greeting_file = File::open("hello.txt").unwrap();
//     let greeting_file = File::open("hello.txt")
//         .expect("hello.txt should be included in this project");
// }



// pub fn read_username_from_file() -> Result<String, io::Error> {
//     // // #3-1
//     // let mut username_file = match File::open("hello.txt") {
//     //     Ok(file) => file,
//     //     Err(e) => return Err(e),
//     // };

//     // let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(MyError::from(e)),
    }

//     // #3-2
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

//#3-3
#[derive(Debug)]
pub enum MyError{
    Io(io::Error),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

pub fn read_username_from_file() -> Result<String, MyError> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

pub fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}