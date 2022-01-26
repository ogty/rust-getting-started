mod types;
mod flow;
mod overload;

fn main() {
    // types
    types::types();

    // flow
    flow::flow();
    
    // overload
    overload::abb(10);
    overload::abb(String::from("Feb"));
}
