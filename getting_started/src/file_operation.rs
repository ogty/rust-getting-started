use csv::{Error, Reader, StringRecord};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::{Path, Display};

pub fn read(path: &str) -> String {
    let path: &Path = Path::new(&path);
    let display: Display = path.display();

    let mut file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s: String = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => s
    }
}

pub fn write(path: &str, content: &str) {
    let path: &Path = Path::new(&path);
    let display: Display = path.display();

    let mut file: File = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => (),
    }
}

fn read_lines_inner_process<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines(path: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines_inner_process(path) {
        for line in lines {
            if let Ok(ip) = line {
                result.push(ip);
            }
        }
    }
    return result;
}

pub fn read_csv(path: &str, column_length: i32) -> Result<HashMap<usize, Vec<String>>, Error> {
    let path: &Path = Path::new(&path);
    let display: Display = path.display();

    let mut file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s: String = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => read_csv_inner_process(s, column_length),
    }
}

fn read_csv_inner_process(data: String, column_length: i32) -> Result<HashMap<usize, Vec<String>>, Error> {
    let mut result: HashMap<usize, Vec<String>> = HashMap::new();
    let rows: usize = data.match_indices("\n").count();
    let mut reader:Reader<&[u8]> = csv::Reader::from_reader(data.as_bytes());

    for record in reader.records() {
        let record: StringRecord = record?;
        for row in 0..rows {
            let mut tmp: Vec<String> = Vec::new();
            for i in 0..column_length {
                tmp.push(format!("{}", &record[i.try_into().unwrap()]));
            }
            result.insert(row, tmp);
        }
    }

    Ok(result)
}
