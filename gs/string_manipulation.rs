fn main() {
    let lang: String = String::from("Rust");
    let mut name = "Bob".to_string();
    println!("Hello, {}!", lang); // Hello, Rust!
    println!("Hello, {}!", name); // Hello, Bob!

    let literal: &str = "Hello";
    name = "Charlotte".to_string();
    let sentence1 = literal.to_owned() + ", " + &name;
    let sentence2 = name + " likes " + &lang;
    println!("{}!", sentence1);                        // Hello, Charlotte!
    println!("{}.", sentence2);                        // Charlotte likes Rust.
    println!("{}", format!("{}, {}!", literal, lang)); // Hello, Rust!

    let thinking_face: char = '🤔';
    println!("{}", thinking_face); // 🤔

    for i in literal.chars() {
        println!("{}", i);
    }
    for i in lang.as_str().chars() {
        println!("{}", i);
    }

    let hello: Vec<char> = literal.chars().collect();
    println!("{:?}", hello); // ['H', 'e', 'l', 'l', 'o']

    let char_data = "Hello";
    let mut chars_data = char_data.chars();
    println!("{}", char_data);                  // Hello
    println!("{:?}", chars_data);               // Chars(['H', 'e', 'l', 'l', 'o'])
    println!("{:?}", chars_data.nth(0));        // Some('H')
    println!("{}", chars_data.nth(1).unwrap()); // l
    println!("{:?}", chars_data.next());        // Some('l')
    println!("{:?}", chars_data.next());        // Some('o')

    let chars_data2 = char_data.chars();
    println!("{:?}", chars_data2.rev().collect::<String>()); // "olleH"

    let mut x: String = String::from("");
    x.push_str("Hello");
    x.push('!');
    println!("{}", x);                // Hello!
    x.pop();
    println!("{}", x);                // Hello
    x.remove(4);
    println!("{}", x);                // Hell
    x.retain(|a| a != 'l');
    println!("{}", x);                // He
    println!("{}", x.len());          // 2
    println!("{}", x.repeat(3));      // HeHeHe
    println!("{}", x.to_uppercase()); // HE
    println!("{}", x.to_lowercase()); // he

    let y = x.replace("e", "ello");
    println!("{}", y);                      // Hello
    println!("{}", y.replace("e", "x", 1)); // Hexlo
}