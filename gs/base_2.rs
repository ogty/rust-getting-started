fn main() {
    // function
    let data = [1, 2, 3, 4, 5];
    println!("{}", sum(data)); // 15

    let mean = {
        let data = [1, 2, 3, 4, 5];
        sum(data) / data.len() as i32
    };
    println!("{}", mean); // 3
}

fn sum(data: [i32; 5]) -> i32 {
    let mut result = 0;
    for i in data.iter() {
        result += i;
    }
    return result;
}
