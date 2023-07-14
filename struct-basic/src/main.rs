#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user_1 = User {
        username: String::from("Nhatcuong"),
        email: String::from("Nhatcuong@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("Hello {:?}", user_1)
}
