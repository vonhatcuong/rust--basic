fn main() {
    // let result =square(10);
    // println!("result is {:?}",result);
    let celsius = 30.0;
    let result = convert_to_fahrenheit(celsius);
    
    assert_eq!(result,86.0);
    println!("Test passed");
}

// fn square(x: i32) -> (i32,i32) {
//     return (x,x * x);
//     println!("This line is never executed.");
// }

fn convert_to_fahrenheit(x: f64) -> f64 {
    return (x * 1.8) + 32.0;
}