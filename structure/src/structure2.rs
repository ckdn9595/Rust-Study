//#1
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//#2
struct AlwaysEqual;

fn main() {
    let black = Color(0, 10, 20);
    let origin = Point(30, 40, 50);

    println!("Black color RGB: {}, {}, {}", black.0, black.1, black.2);
    println!("Origin coordinates: {}, {}, {}", origin.0, origin.1, origin.2);

    let subject1 = AlwaysEqual;
    let subject2 = AlwaysEqual;

    if subject1 == subject2{
        println!("Both objects are always equal!");
    }
}