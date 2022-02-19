pub fn string_manipulation() {
    let maxim = String::from("Don't Repeat Yourself");
    let mut maxim_abb = String::new();

    for word in maxim.split_whitespace() {
        maxim_abb.push(word.chars().next().unwrap());
    }
    println!("{}", maxim);
    println!("{}", maxim_abb);
    println!("{}", maxim_abb.len());

    let hello_world = String::from("Hello World!");
    let mut hello_rust = hello_world.replace("World", "Rust");
    
    hello_rust.retain(|c| c != '!');
    println!("{}", hello_world);
    println!("{}", hello_rust);
    // TODO: pop, remove, push_str

    let rate = String::from("+25.8");
    println!("{}", rate.parse::<f32>().unwrap());

    let path = "~/Desktop/project1/main.py";
    let path_splited: Vec<&str> = path.split('.').collect();
    let extension = format!("{:?}", path_splited.last());
    println!("{}", extension);
}
