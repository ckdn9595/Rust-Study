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

// fn string_func(s : &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s1 = String::from("hello");
    
//     let s2 = &mut s1;
//     add_name(s2);
//     // s2의 스코프가 여기서 끝남

//     s1.push_str("- s1 update");
//     println!("{}", s1);

//     let len = string_func(&s1);
//     println!("{}, {}", s1, len);
// }

// fn add_name(s : &mut String) {
//     s.push_str(", jiwoo");
// }

fn extract_domain(url: &str) -> &str {
    let after_protocol = if url.starts_with("http://") {
        &url["http://".len()..]
    } else if url.starts_with("https://") {
        &url["https://".len()..]
    } else {
        url
    };

    let domain_end = after_protocol.find('/').unwrap_or(after_protocol.len());
    &after_protocol[..domain_end]
}


fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    let mut url = String::from("https://ziwoodev.tistory.com/37");
    let domain = extract_domain(&url);
    url.push_str("0");
    println!("Domain name: {}", domain);
    println!("{}", url);
}