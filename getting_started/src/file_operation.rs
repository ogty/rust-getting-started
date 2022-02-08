use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;


pub fn read(path: String) -> String {
    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => s,
    }
}

pub fn write(path: String, content: &str) {
    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => (),
    }
}

fn read_lines_inner_process<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines(path: String) -> Vec<String> {
    let mut result = Vec::new();
    if let Ok(lines) = read_lines_inner_process(path) {
        for line in lines {
            if let Ok(ip) = line {
                result.push(ip);
            }
        }
    }
    return result;
}