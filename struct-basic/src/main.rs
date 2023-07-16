#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn get_username(&self) -> &String {
        &self.username
    }

    fn new(username: &str, email: &str) -> User {
        User {
            username: String::from(username),
            email: String::from(email),
            sign_in_count: 1,
            active: true,
        }
    }

}

fn main() {
    let user_1 = User::new("Nhatcuong", "vonhatcuong@gmail.com");

    let user_2 = User {
        // username: String::from("Nhatcuong"),
        email: String::from("aaaa@gmail.com"),
        .. user_1
    };
    let username = user_2.get_username();

    
    println!("Hello {:?}", username);
    // println!("Hello {:?}", user_1.active);
}
