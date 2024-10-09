use std::env;

#[derive(Debug, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Copy)]
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