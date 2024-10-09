// mod variables;
// mod data_type;

// fn main() {
//     //variables::test4();
//     data_type::test3();
// }
use std::env;

#[derive(Debug, Clone)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
   let me = Employee {
        position: Position::Worker,
        work_hours: 40,
   };
   print_employee(me);
}