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
use crate::schedule_generator::{Components, ScheduleGenerator};
mod run_command;
mod whoami;
mod default_arguments;
use crate::default_arguments::{Greeting, MessageTemplates};
mod macros;

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
    let mut numbers: [i32; 13] = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];
    println!("{}", basic_stats::average(&numbers));
    println!("{}", basic_stats::median(&mut numbers));
    println!("{}", basic_stats::mode(&numbers));

    // structure
    structure::main();

    // enum
    enumerated::main();

    // read, write, read lines
    let hello: String = file_operation::read("./data/hello.txt");
    println!("{}", hello);
    
    file_operation::write("./data/maxim.txt", MAXIM);
    let file_contents_array: Vec<String> = file_operation::read_lines("./data/maxim.txt");
    println!("{:?}", file_contents_array[0]);

    let df = file_operation::read_csv("./data/stock.csv", 6);
    println!("{:?}", df);

    // plotting
    let data: Vec<u32> = vec![0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3];
    let result = plot::main(data);
    println!("{:?}", result);

    // schedule generator
    let mut generator: ScheduleGenerator = schedule_generator::ScheduleGenerator{ start: 9, end: 18, ..Default::default() };
    generator.generate();
    println!("{:?}", generator.time_schedules);
    
    generator.addition(vec!["07:00", "08:00"]);
    println!("{:?}", generator.time_schedules);

    // run command
    run_command::run();

    // whoami
    let iam = whoami::whoami();
    println!("My name is {}", iam); 
    println!("{}", whoami::desktop());

    // default argument
    let mut greeter = Greeting::default();
    greeter.general_message();

    greeter.name = "John";
    greeter.general_message();

    let mut greeter2 = Greeting{ ..Default::default() };
    greeter2.name = "Bob";
    greeter2.general_message();

    let mut greeter3 = Greeting{ name: "Alice", ..Default::default() };
    greeter3.general_message();

    // macro
    macros::main();
}

static MAXIM: &str =
    "KISS: Keep It Simple, Stupid
DRY: Don't Repeat Yourself
YAGNI: You Aren't Going to Need It
SLAP: Single Level of Abstraction Principle
OCP: Open-Closed Principle";
