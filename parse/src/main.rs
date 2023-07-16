use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Type something: ");
     io::stdin().read_line(&mut buffer).expect("Failed");
     println!("You typed: {}", buffer.trim());

     let number: i32= buffer.trim().parse().unwrap();
     println!("number +1 is {}", number + 1)
}
