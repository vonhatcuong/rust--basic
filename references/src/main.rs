fn main() {
   let rocket_fuel = produce_fuel();
    println!("Fuel produced: {}", rocket_fuel);
}

fn produce_fuel() -> String{
    let new_fuel = String::from("Fuel");
    new_fuel
}