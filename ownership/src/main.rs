// fn main() {
//     let mut s1 = String::from("hello");
//     let s2 = &mut s1;
//     add_name(s2);
//     s1.push_str("- s1 update");
//     println!("{}", s2);

//     let len = string_func(&s1);

//     println!("{}, {}", s1, len);
// }

// fn add_name(s : &mut String) {
//     s.push_str(", jiwoo");
// }

fn string_func(s : &String) -> usize {
    s.len()
}

fn main() {
    let mut s1 = String::from("hello");
    {
        let s2 = &mut s1;
        add_name(s2);
    } // s2의 스코프가 여기서 끝남

    s1.push_str("- s1 update");
    println!("{}", s1);

    let len = string_func(&s1);
    println!("{}, {}", s1, len);
}

fn add_name(s : &mut String) {
    s.push_str(", jiwoo");
}