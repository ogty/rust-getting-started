mod types;
mod overload;

fn main() {
    // types
    types::types();

    // overload
    overload::abb(10);
    overload::abb(String::from("Feb"));
}
