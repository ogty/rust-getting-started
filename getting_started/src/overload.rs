use std::collections::HashMap;

pub trait Abb<T> {
    fn abb(&mut self, c:T);
}

pub struct AbbImpl {
    month_word: Vec<String>,
    month_num: Vec<i32>,
}

impl Abb<i32> for AbbImpl {
    fn abb(&mut self, c:i32) {
        let months: HashMap<_, _> = self.month_num.iter().zip(self.month_word.iter()).collect();
        match months.get(&c) {
            Some(v) => println!("{}", v),
            None => println!("error"),
        };
    }
}

impl Abb<String> for AbbImpl {
    fn abb(&mut self, c:String) {
        let months: HashMap<_, _> = self.month_word.iter().zip(self.month_num.iter()).collect();
        match months.get(&c) {
            Some(v) => println!("{}", v),
            None => println!("error"),
        };
    }
}

pub fn abb<T>(data:T) where AbbImpl:Abb<T> {
    let month_word = vec![
        String::from("Jan"), String::from("Feb"), String::from("Mar"),
        String::from("Apr"), String::from("May"), String::from("Jun"),
        String::from("Jul"), String::from("Aug"), String::from("Sep"),
        String::from("Oct"), String::from("Nov"), String::from("Dec")
    ];
    let month_num = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let mut abb = AbbImpl{month_word, month_num};
    Abb::abb(&mut abb, data);
}
