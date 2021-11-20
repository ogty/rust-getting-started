/*
 - Use underscores as a prefix for unused variables.
*/

fn main() {
    // string
    let s1: String = String::from("Hello, world!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("{}", type_of(&s1)); // &alloc::string::String
    println!("{}", type_of(&s2)); // &&str
    println!("{}", type_of(&s3)); // &alloc::string::String


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


    // struct
    struct Person {
        name: String,
        age: u32,
    }

    let p = Person {
        name: String::from("John"),
        age: 8,
    };
    println!("name: {}\nage: {}", p.name, p.age);
    // name: John
    // age: 8
}

fn type_of<T>(_: T) -> String {
    let data = std::any::type_name::<T>();
    return data.to_string();
}