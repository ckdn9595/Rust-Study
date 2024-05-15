fn main() {
    //let x = {let y = 1; y + 1;};
    //let x = { let y = 1; y + 1 };
    //println!("{x}");
    let num = 5;
    let day_unit = "월";
    test_void(num, day_unit);
    println!("{}", test_return(num, day_unit));
    test_if(num);
}
fn test_void(num: u32, day_unit: &str) {
    println!("1 오늘은 {num}{day_unit}입니다.");
}
fn test_return(num: u32, day_unit: &str) -> String {
    let str = format!("2 오늘은 {}{}입니다.", num, day_unit);
    str
}

fn test_if(num: u32){
    if num > 1 {
        println!("true");
    }else{
        println!("false");
    }

    if num % 2 == 0 {
        println!("num is divisible by 2");
    } else if num % 1 == 0 {
        println!("num is divisible by 1");
    } else {
        println!("num is not divisible");
    }

    let bigger_num = if num > 6 {num} else {6};
    println!("The bigger_num is: {bigger_num}");
}
