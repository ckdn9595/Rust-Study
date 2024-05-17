fn main() {
    let x = {let y = 1; y};
    let z = { let y = 1; y + 1 };
    println!("{x}");
    println!("{z}");
    let num = 5;
    let day_unit = "월";
    test_void(num, day_unit);
    println!("{}", test_return(num, day_unit));
    test_if(num);
    let count = test_loop();
    println!("the loop test result is {}", count);
    test_multiple_loop();
    test_for();
}
fn test_void(num: u32, day_unit: &str) {
    println!("1 오늘은 {num}{day_unit}입니다.");
}
fn test_return(num: u32, day_unit: &str) -> String {
    let str = format!("2 오늘은 {}{}입니다.", num, day_unit);
    str
}

fn test_if(num: u32) {
    if num > 1 {
        println!("true");
    } else {
        println!("false");
    }

    if num % 2 == 0 {
        println!("num is divisible by 2");
    } else if num % 1 == 0 {
        println!("num is divisible by 1");
    } else {
        println!("num is not divisible");
    }

    let bigger_num = if num > 6 { num } else { 6 };
    println!("The bigger_num is: {bigger_num}");
}

fn test_loop() -> u32 {
    let mut count = 0;
    loop {
        //count++; <- 이거 안됨
        count += 1;
        if count == 10 {
            break count;
        }
    }
}

fn test_multiple_loop() {
    // 1 ~ 4 를 가진 카드 4장에서 2장을 뽑는 이중 포문
    let cards: [u8; 4] = [1, 2, 3, 4];
    let mut cnt = 0;
    let mut a_index = 0;
    let mut b_index = a_index + 1;
    'first: loop {
        loop {
            if a_index == 4 {
                break 'first;
            }
            if b_index == 4 {
                break;
            }
            cnt += 1;
            println!("{}번째 : {}, {}", cnt, cards[a_index], cards[b_index]);
            b_index += 1;
        }
        a_index += 1;
        b_index = a_index + 1;
    }
}
fn test_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn test_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4) {
        println!("{number}!");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}
