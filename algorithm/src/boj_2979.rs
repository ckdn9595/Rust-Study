use std::io::{self, BufRead};

#[derive(Debug)]
struct Truck{
    start: usize,
    end: usize,
    number: String,
    active: bool,
}

impl Truck {
    fn new(start:usize, end: usize, number:i32) -> Self{
        Self{
            start,
            end, // <- 축약 가능
            number: format!("Truck {}", number+1),
            active: true,
        }
    }
    fn calc(&self, truck_count: &mut [i32; 101]){
        for time in self.start..self.end {
            truck_count[time] += 1;
        }
    }
}


struct Point(i8,i32,i32,i32);

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let costs = lines.next().unwrap().unwrap();
    let costs: Vec<i32> = costs.split_whitespace()
                               .map(|s| s.parse().unwrap())
                               .collect();

    let costs = Point(0, costs[0], costs[1], costs[2]);

    let mut truck_count = [0; 101]; 

    for idx in 0..3 {
        let times = lines.next().unwrap().unwrap();
        let times: Vec<usize> = times.split_whitespace()
                                     .map(|s| s.parse().unwrap())
                                     .collect();

        let truck = Truck::new(times[0], times[1], idx);
        truck.calc(&mut truck_count);
    }

    let mut total_cost = 0;
    for time in 1..=100 {
        total_cost += match truck_count[time] {
            1 => costs.1,
            2 => costs.2 * 2,
            3 => costs.3 * 3,
            _ => 0,
        };
    }
   
    println!("{}", total_cost);
}