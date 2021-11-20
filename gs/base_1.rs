/*
 - Use underscores as a prefix for unused variables.
*/

fn main() {
    // type
    /*
    String
    char
    bool
    i: 8, 16, 32, 64
    u: 8, 16, 32, 64
    f: 32, 64

    10base : 1_000_000   = 1000000
    16base : 0xff        = 255
    8base  : 0o77        = 63
    2base  : 0b1111_0000 = 240
    */

    // string
    let s1: String = String::from("Hello, world!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("{}", type_of(&s1)); // &alloc::string::String
    println!("{}", type_of(&s2)); // &&str
    println!("{}", type_of(&s3)); // &alloc::string::String


    // char
    let thinking_face = '🤔';
    println!("{}", type_of(thinking_face)); // char


    // bool
    let t: bool = true;
    let f: bool = false;
    println!("{}", type_of(t)); // bool
    println!("{}", type_of(f)); // bool


    // tuple
    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    println!("{}", type_of(t.0)); // i32
    println!("{}", type_of(t.1)); // &str


    // array
    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    println!("{:?}", a); // [0, 1, 2]
    println!("{:?}", b); // [0, 0, 0]

    a[1] = b[1];
    a[2] = b[2];
    println!("{:?}", a);        // [0, 0, 0]
    println!("{:?}", b);        // [0, 0, 0]
    println!("{:?}", &a[1..3]); // [0, 0]
    println!("{:?}", &a[..3]);  // [0, 0, 0]
    /*
    To use the slice, you need to address it.
    The 0 in the slice can be omitted.
    The end can be omitted as well.
    */
 

    // if
    let age = 20;
    if age >= 20 {
        println!("You can drink!");
    } else if age == 19 {
        println!("Let's hold out for one more year.")
    } else {
        println!("...");
    }

    let animal = "cat".to_string();
    let food = if animal == "cat" {
        "tuna"
    } else {
        "Let's find out."
    };
    
    println!("{}", food); // tuna

    
    // loop, while and for
    let mut count1 = 0;
    loop {
        println!("{}", count1);
        if count1 == 5 {
            break;
        } else {
            count1 += 1;
        }
    }

    let mut count2 = 0;
    while count2 < 6 {
        println!("{}", count2);
        if count2 == 5 {
            break;
        } else {
            count2 += 1;
        }
    }

    let data = [1, 2, 3, 4, 5];
    for i in data.iter() {
        println!("{}", i);
    }
    for i in 0..6 {
        println!("{}", i)
    }

    // 0
    // 1
    // 2
    // 3
    // 4
    // 5

    // struct
    struct Person {
        name: String,
        age: u32,
    }

    let p = Person {
        name: String::from("John"), // "John".to_string()
        age: 8,
    };
    println!("name: {}\nage: {}", p.name, p.age);
    // name: John
    // age: 8


    // Option
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);
    println!("{:?}", x); // Some(2)
    
    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);
    println!("{:?}", x); // None
}

fn type_of<T>(_: T) -> String {
    let data = std::any::type_name::<T>();
    return data.to_string();
}