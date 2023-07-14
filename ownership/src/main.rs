fn main() {
    let s = String::from("hello");
    let (s, length) = create_data(s);
    println!("s: {}, length {}", s, length);
}

fn create_data(data: String) -> (String, usize) {
    println!("data: {}", data);
    // let new_daa = String::from("world");
    let length = data.len();
    (data, length)
}
