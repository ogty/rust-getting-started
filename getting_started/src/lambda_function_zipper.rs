use std::env;
use std::process::{Command};


fn run(command: &str, arguments: Vec<&str>) {
    Command::new(command)
        .args(arguments)
        .output()
        .expect("failed to execute process");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let requirements_path = &format!("./{}/requirements.txt", path);

    run("pip3", vec!["install", "-r", requirements_path, "-t", path]);

    if cfg!(windows) {
        run("powershell", vec!["compress-archive", path, path])
    } else {
        run("zip", vec!["-r", &format!("./{}.zip", path), path])
    }
}
