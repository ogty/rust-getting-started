use std::vec;

mod types;
mod flow;
mod overload;
mod string_manipulation;
mod basic_stats;
mod structure;
mod enumerated;
mod file_operation;
mod plot;
mod schedule_generator;
use crate::schedule_generator::Components;

fn main() {
    // types
    types::types();

    // flow
    flow::flow();
    
    // overload
    println!("{}", overload::abb(10));
    println!("{}", overload::abb("Feb"));

    // string manipulation
    string_manipulation::string_manipulation();

    // basic statistics 
    let mut numbers = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];
    println!("{}", basic_stats::average(&numbers));
    println!("{}", basic_stats::median(&mut numbers));
    println!("{}", basic_stats::mode(&numbers));

    // structure
    structure::main();

    // enum
    enumerated::main();

    // read, write, read lines
    let hello = file_operation::read("./data/hello.txt");
    println!("{}", hello);
    
    file_operation::write("./data/maxim.txt", MAXIM);
    let file_contents_array = file_operation::read_lines("./data/maxim.txt");
    println!("{:?}", file_contents_array[0]);

    let df = file_operation::read_csv("./data/stock.csv", 6);
    println!("{:?}", df);

    // plotting
    let data = vec![0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3];
    let result = plot::main(data);
    println!("{:?}", result);

    // schedule generator
    let mut generator = schedule_generator::ScheduleGenerator{ start: 9, end: 18, ..Default::default() };
    generator.generate();
    println!("{:?}", generator.time_schedules);
    
    generator.addition(vec!["07:00", "08:00"]);
    println!("{:?}", generator.time_schedules);
}

static MAXIM: &str =
    "KISS: Keep It Simple, Stupid
DRY: Don't Repeat Yourself
YAGNI: You Aren't Going to Need It
SLAP: Single Level of Abstraction Principle
OCP: Open-Closed Principle";
