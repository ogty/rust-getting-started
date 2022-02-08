struct Url {
    scheme: String,
    netloc: String,
    path: String,
    params: String,
    query: String,
    fragment: String
}

struct Position(f32, f32, f32);

pub fn main() {
    // "https://example.com/items?page=3&sort=popular"
    let url = Url {
        scheme: "https".to_string(),
        netloc: "example.com".to_string(),
        path: "items".to_string(),
        params: "page=3&sort=popular".to_string(),
        query: "".to_string(),
        fragment: "".to_string()
    };
    println!("{}", url.netloc);

    let Position (x, y, z) = Position(-199.036, 75.0000, -2376.8457);
    println!("XYZ: {} / {} / {}", x, y, z);
}
