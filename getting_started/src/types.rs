pub fn types() {
    // int
    let inum: i32 = 100;
    let fnum: f64 = 1.1;
    type_of(inum);
    type_of(fnum);

    // string
    let s1: String = String::from("Hello, world!");
    let s2: &str = &s1;
    type_of(&s1);
    type_of(&s2);
    
    // char
    let thinking_face: char = '🤔';
    type_of(thinking_face);
    
    // bool
    let t: bool = true;
    type_of(t);

    // tuple
    let t: (i32, &str) = (1, "2");
    type_of(t);

    // array
    let a: [i32; 3] = [0, 1, 2];
    type_of(a);
}

fn type_of<T>(_: T) {
    let data: &str = std::any::type_name::<T>();
    println!("{}", data);
}
