// logistic struct definition
#[derive(Debug)]
struct Logistic {
    pub name: String,
    pub age: u8,
}


// logistic struct methods
impl Logistic {
    // constructor
    fn new(name: String, age: u8) -> Logistic {
        Logistic { name, age }
    }

    // predict
    fn predict(&self, x: f64) -> f64 {
        let mut y = 0.0;

        if x < self.age as f64 {
            y = 1.0;
        }

        y
    }
}


// main
pub fn main() {
    // logistic struct definition
    let logistic = Logistic::new(String::from("John"), 30);

    // predict
    println!("{}", logistic.predict(30.0));
}

