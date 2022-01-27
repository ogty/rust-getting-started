mod types;
mod flow;
mod overload;
mod string_manipulation;
mod guessing_game;

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

    guessing_game::main();
}
