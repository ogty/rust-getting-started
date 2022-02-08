mod types;
mod flow;
mod overload;
mod string_manipulation;
mod basic_stats;
mod structure;

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
}
