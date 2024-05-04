// pub fn test1() {
//     let x : i32 = "456".parse().expect("not a number");
//     println!("{}", x);
// }
// pub fn test2() {
//     let x = 789;
//     println!("{}", x);
// }
use std::collections::HashMap;

pub fn test3() {
    let x = (1, '1', "일");
    println!("{}, {}, {}", x.0, x.1, x.2);

    let (a,b,c) = x;
    println!("{}, {}, {}", a, b, c);

    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    println!("{}, {}", days[0], days[6]);

    let arr = [1,2,3,4,5];
    let arr : [i32;5] = [1,2,3,4,5];
    let arr = [1;5];
    dbg!(arr);


    let mut typeString : String = String::from("hello");
    typeString.push_str(", world!");
    println!("{}", typeString);

    let typeStr : &str = "Hello, world!"; 
    println!("{}", typeStr);  

    let mut tStr : &str = "first";
    tStr = "second";
    println!("{}", tStr);  

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", vec); // [1, 2, 3]

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    println!("{:?}", scores);
}