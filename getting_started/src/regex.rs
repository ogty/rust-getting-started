use regex::Regex;

pub fn main() {
    let re1 = Regex::new(r"\d+").unwrap();
    let text1 = "123456789";
    println!("{:?}", re1.find(text1));
    println!("{:?}", re1.captures(text1));

    let re2 = Regex::new(r"(?P<first>\d+)\.(?P<second>\d+)").unwrap();
    let text2 = "123.456";
    println!("{:?}", re2.captures(text2));


    // regex to get between two strings
    let re3 = Regex::new(r"(?P<first>\w+)\s(?P<second>\w+)").unwrap();
    let text3 = "hello world";
    println!("{:?}", re3.captures(text3));


    // zip code regex
    let re4 = Regex::new(r"(?P<zip>\d{5})\s(?P<city>\w+)").unwrap();
    let text4 = "12345 Boston";
    println!("{:?}", re4.captures(text4));


    // phone number regex
    let re5 = Regex::new(r"(?P<area>\d{3})\s(?P<prefix>\d{3})\s(?P<suffix>\d{4})").unwrap();
    let text5 = "123 456 7890";
    println!("{:?}", re5.captures(text5));


    // date regex
    let re7 = Regex::new(r"(?P<month>\w+)\s(?P<day>\d+)\s(?P<year>\d+)").unwrap();
    let text7 = "January 1 2000";
    println!("{:?}", re7.captures(text7));


    // time regex
    let re8 = Regex::new(r"(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})").unwrap();
    let text8 = "12:34:56";
    println!("{:?}", re8.captures(text8));


    // date and time regex
    let re9 = Regex::new(r"(?P<month>\w+)\s(?P<day>\d+)\s(?P<year>\d+)\s(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})").unwrap();
    let text9 = "January 1 2000 12:34:56";
    println!("{:?}", re9.captures(text9));
}