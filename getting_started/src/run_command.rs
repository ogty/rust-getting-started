use std::process::{Command, Output};

pub fn run() {
    let output: Output = Command::new("julia")
        .args(["-e", "using Pkg; Pkg.status();"])
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8(output.stdout).unwrap());
}
