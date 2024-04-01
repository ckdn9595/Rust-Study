use std::io::{self, Read};
use std::fs::File;
use std::io::ErrorKind;
fn string_len(s:&String) -> usize{
    println!("{s}");
    let len = s.len();
    len
}
#[derive(Debug, PartialEq)] //PartialEq == 비교 연산 디버그 할때 필요
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn area2(&self) -> u32{
        self.width * self.height
    }
}

enum Message{
    StartGame,
    WinPoint {who:String},
    ChangePlayerName(String),
}

fn handle_message(message: &Message) {
    match message {
        Message::StartGame => println!("startGame"),
        Message::WinPoint { who } => println!("{}", who),
        _ => println!("not impl")
    }
}
// enum Result<T,E>{
//     OK(T),
//     ERR(E)
// }

fn read_ddd() -> File {
    let file = File::open("hello.txt").expect("ssss");

    let file = match file {
        Ok(f) => f,
        Err(e) => return File.None,
    }
}
fn main(){
    //let file = File::open("hello.txt");
    
    

    let v = file.ok();
    let mut username = String::new();
    if let Some(v) = v{
        v.read_to_string(&mut username)?;
        Ok(username);
    }

    let rect = Rectangle{
        width: 20,
        height: 30
    };

    println!("사각형 = {:?}", rect);
    println!("{}",rect.width);
    //dbg!(rect);
    println!("{}",rect.area2());
    // let s = String::from("헬로");
    // let len = string_len(&s);
    // println!("{s} : {len}");
    // let str = "hi";
    // let mut str = "hi";
    // str = "ss";


    
    // let mut user_input = String::from("hi");
    // user_input.push_str("string");

    // let t = (32, true, 3.14);
    // let x: i32 = t.0;
    // let y: bool = t.1;
    // let z = t.2;
    // let tuple = t;
    // a_function(3);
    // let arr:[i32; 5] = [1,2,3,4,5]; //배열은 동일한 타입만 // 러스트는 배열의 길이가 고정

    // let threes: [&str; 10] = ["헬로"; 10];
    // //println(1);
    // println!("{:?}", threes);

    // let x = {
    //     let x = 8; //명령문 
    //     1111            //식  => 식만 반환값이 될 수 있다.
    // };
    // println!("x = {x}");
    // let area = circle_area(5.0);
    // println!("{area}");

    // let x = 4;
    // if x%2==0 {
    //     println!("x는 짝수")
    // }else {
    //     println!("홀수")
    // }

    // let mut counter = 0;
    // let mut x = loop{ 
    //     counter+=1;
    //     if counter == 3{
    //         break counter
    //     }
    // };
    
    // println!("x = {x}");
    // let arr = [1,2,3,4,5,6,7,8];
    // while x < arr.len(){
    //    println!("x = {x}");
    //    x += 1;
    // }

    // for x in arr {
    //     println!("x = {x}");
    // }
    // for x in (1..8).rev() {
    //     println!("x = {x}");
    // }

    // let mut str:String = String::from("헬로");
    // str.push_str(", wldn string");
    // println!("x = {str}");
}
const PI:f64 = 3.141592;
fn circle_area(radius: f64) -> f64{
    PI * radius * radius
}


fn a_function(x: i32){
    println!("a_function  {x}");
}
// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

// fn main() {
//     println!("가위바위보 게임에 오신 것을 환영합니다!");

//     let choices = ["가위", "바위", "보"];
//     let computer_choice = rand::thread_rng().gen_range(0..choices.len());

//     println!("가위, 바위, 보 중 하나를 선택하세요:");

//     let mut user_input = String::new();
//     io::stdin()
//         .read_line(&mut user_input)
//         .expect("입력을 읽는 데 실패했습니다.");

//     let user_input = user_input.trim();
//     let user_choice = choices.iter().position(|&r| r == user_input);

//     match user_choice {
//         Some(index) => {
//             println!("당신은 {}를 선택했습니다.", user_input);
//             println!("컴퓨터는 {}를 선택했습니다.", choices[computer_choice]);

//             match (index, computer_choice) {
//                 (i, c) if i == c => println!("비겼습니다!"),
//                 (0, 2) | (1, 0) | (2, 1) => println!("당신이 이겼습니다!"),
//                 _ => println!("컴퓨터가 이겼습니다!"),
//             }
//         },
//         None => println!("잘못된 선택입니다."),
//     }
// }