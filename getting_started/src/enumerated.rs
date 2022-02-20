pub fn main() {
    let m: Message = Message::Move{ x: 0, y: 0 };
    m.call();
    Message::Move{ x: 1, y: 1 }.call();
    Message::Quit.call();
    Message::Write(String::from("Hello")).call();
    Message::ChangeColor(255, 0, 0).call();
}

#[derive(PartialOrd, PartialEq, Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("call {:?}", self);
    }
}
