use std::io::{self, BufRead};

#[derive(Debug)]
struct Truck{
    start: usize,
    end: usize,
    number: String,
    active: bool,
}

//#7
impl Truck {
    //#8
    fn new(start:usize, end: usize, number:i32) -> Self{
        Self{
            start,
            end, // <- 축약 가능
            number: format!("Truck {}", number+1),
            active: true,
        }
    }
}
impl Truck {
    fn calc(&self, truck_count: &mut [i32; 101]){
        for time in self.start..self.end {
            truck_count[time] += 1;
        }
    }
}

//#5
struct Point(i8,i32,i32,i32);
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
    let mut num : i32 = 2;

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let costs = lines.next().unwrap().unwrap();
    let costs: Vec<i32> = costs.split_whitespace()
                               .map(|s| s.parse().unwrap())
                               .collect();
    let a = (costs[0], costs[1]);
    let a = (costs[0], costs[1], costs[2]);
    let a = (costs[0], costs[1], costs[2]);
    let a = (costs[0], costs[1], costs[2]);
    let point = Point(0, costs[0], costs[1], costs[2]);

    let mut truck_count = [0; 101]; 

    for idx in 0..3 {
        let times = lines.next().unwrap().unwrap();
        let times: Vec<usize> = times.split_whitespace()
                                     .map(|s| s.parse().unwrap())
                                     .collect();
    
       
        //#3
        let truck = Truck::new(times[0], times[1], idx);
        // for time in truck.start..truck.end {
        //     truck_count[time] += 1;
        // }
        //#4
        // let truck2 = Truck{
        //     active:false,
        //     ..truck
        // };
        // let truck2 = Truck{
        //     number:String::from("new one"),
        //     ..truck
        // };
        //#6
        //println!("truck is {:#?}", truck);
        //println!("truck is {:?}", truck);
        //let truck = dbg!(truck);
        //dbg!(&truck);
        //dbg!(&truck2);
        
        // for time in truck.start..truck.end {
        //     truck_count[time] += 1;
        // }

        //#7
        truck.calc(&mut truck_count);
    }

    let mut total_cost = 0;
    for time in 1..=100 {
        total_cost += match truck_count[time] {
            1 => point.1,
            2 => point.2 * 2,
            3 => point.3 * 3,
            _ => 0,
        };
    }
   
    println!("{}", total_cost);
}