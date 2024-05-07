use std::io::{self, BufRead};

#[derive(Debug)]
struct Truck{
    start: usize,
    end: usize,
    number: String,
    active: bool,
}
//#5
//struct Point(i8,i32,i32,i32);
//#3
fn build_truck(start: usize, end: usize, number:i32) -> Truck{
    Truck{
        start: start,
        end, // <- 축약 가능
        number: format!("Truck {}", number+1),
        active: true,
    }
}

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let costs = lines.next().unwrap().unwrap();
    let costs: Vec<i32> = costs.split_whitespace()
                               .map(|s| s.parse().unwrap())
                               .collect();
    let (a, b, c) = (costs[0], costs[1], costs[2]);
    //let point = Point(0, costs[0], costs[1], costs[2]);

    let mut truck_count = [0; 101]; 

    for idx in 0..3 {
        let times = lines.next().unwrap().unwrap();
        let times: Vec<usize> = times.split_whitespace()
                                     .map(|s| s.parse().unwrap())
                                     .collect();
        //#1
        // 튜플
        //let (start, end) = (times[0], times[1]); 

        // for time in start..end {
        //     truck_count[time] += 1;
        // }
        
        //#2
        // let mut truck = Truck{
        //     start: times[0],
        //     end: times[1],
            
        //     number: format!("Truck {}", idx+1),
        // };
        // truck.number = "AI ".to_owned() + &truck.number; //to_owned() 문자열 리터럴인 &str을 String 타입으로 변환
        // println!("{}", truck.number);

        //#3
        let truck = build_truck(times[0], times[1], idx);
        //#4
        // let truck2 = Truck{  // 1
        //     active:false,
        //     ..truck
        // };
        // let truck2 = Truck{  // 2 
        //     number:String::from("new one"),
        //     ..truck
        // };
        //#?
        //dbg!(truck);
        dbg!(&truck);
        //dbg!(&truck2);
        
        for time in truck.start..truck.end {
            truck_count[time] += 1;
        }
    }

    let mut total_cost = 0;
    for time in 1..=100 {
        total_cost += match truck_count[time] {
            1 => a,
            2 => b * 2,
            3 => c * 3,
            _ => 0,
        };
    }
    //#5
    // for time in 1..=100 {
    //     total_cost += match truck_count[time] {
    //         1 => point.1,
    //         2 => point.2 * 2,
    //         3 => point.3 * 3,
    //         _ => 0,
    //     };
    // }
    println!("{}", total_cost);
}