mod types;
mod flow;
mod overload;
mod string_manipulation;
mod basic_stats;
mod structure;
mod enumerated;
mod file_operation;

fn main() {
    // types
    types::types();

    // flow
    flow::flow();
    
    // overload
    overload::abb(10);
    overload::abb(String::from("Feb"));

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
    let hello = file_operation::read("./data/hello.txt".to_string());
    println!("{}", hello);
    file_operation::write("./data/maxim.txt".to_string(), MAXIM);
    let file_contents_array = file_operation::read_lines("./data/maxim.txt".to_string());
    println!("{:?}", file_contents_array[0]);
}

static MAXIM: &str =
    "KISS: Keep It Simple, Stupid
DRY: Don't Repeat Yourself
YAGNI: You Aren't Going to Need It
SLAP: Single Level of Abstraction Principle
OCP: Open-Closed Principle";
