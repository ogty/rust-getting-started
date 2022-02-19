use std::collections::HashMap;
use std::fmt;

struct Url {
    _scheme: String,
    netloc: String,
    _path: String,
    _params: String,
    _query: String,
    _fragment: String
}

struct Position(f32, f32, f32);

struct Password(String);

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.chars().map(|_| '*').collect::<String>())
    }
}

struct MyCalc {
    data: Vec<i32>
}

impl MyCalc {
    fn average(&self) -> f32 {
        self.data.iter().sum::<i32>() as f32 / self.data.len() as f32
    }
}

impl MyCalc {
    fn median(&mut self) -> i32 {
        self.data.sort();
        let mid = self.data.len() / 2;
        self.data[mid]
    }

    #[allow(dead_code)]
    fn mode(&self) -> i32 {
        let mut occurrences = HashMap::new();
    
        for &value in &self.data {
            *occurrences.entry(value).or_insert(0) += 1;
        }
    
        occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .expect("Cannot compute the mode of zero numbers")
    }
}

pub fn main() {
    // "https://example.com/items?page=3&sort=popular"
    let url = Url {
        _scheme: "https".to_string(),
        netloc: "example.com".to_string(),
        _path: "items".to_string(),
        _params: "".to_string(),
        _query: "page=3&sort=popular".to_string(),
        _fragment: "".to_string()
    };

    println!("{}", url.netloc);

    let Position (x, y, z) = Position(-199.036, 75.0000, -2376.8457);
    println!("XYZ: {} / {} / {}", x, y, z);

    let username = "admin";
    let password = Password("password".to_string());
    println!("username: {}", username);
    println!("password: {}", password);

    let array_1 = vec![1, 2, 3, 4, 5];
    let array_2 = vec![5, 6, 7, 8, 9];

    let instance_1 = MyCalc { data: array_1 };
    let mut instance_2 = MyCalc { data: array_2 };
    println!("The average for array_1 is {}", instance_1.average());
    println!("The median for array_2 is {}", instance_2.median());
}
