pub fn flow() {
    let age: i32 = 20;
    
    if age > 76 {
        println!("Old age");
    } else if age > 45 {
        println!("Middle age");
    } else if age > 17 {
        println!("Youth");
    } else if age > 5 {
        println!("Boy");
    } else {
        println!("Childhood");
    }

    let split_word: &str = if cfg!(target_os = "windows") { "\\" } else { "/" };
    println!("{}", split_word);

    let mut count: i32 = 0;
    let result: i32 = loop {
        count += 1;
        if count == 5 {
            break count
        }
    };
    println!("{}", result);

    let mut num: i32 = 3;
    while num == 0 {
        println!("{}", num);
        num -= 1;
    };

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    for i in array.iter() {
        println!("{}", i);
    }
}
