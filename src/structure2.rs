struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 10, 20);
    let origin = Point(30, 40, 50);

    println!("Black color RGB: {}, {}, {}", black.0, black.1, black.2);
    println!("Origin coordinates: {}, {}, {}", origin.0, origin.1, origin.2);
}