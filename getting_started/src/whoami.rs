use std::process::{Command, Output};

pub fn whoami() -> Vec<String> {
    let output: Output = Command::new("whoami")
        .output()
        .expect("failed to execute process");
    let info = String::from_utf8(output.stdout).unwrap();
    let computer = info.split("\\").collect::<Vec<&str>>()[0];
    let username = info.split("\\").collect::<Vec<&str>>()[1];
    let result = vec![computer.to_string(), username.to_string()];
    return result
}
