fn main() {
    let stuff = (10,3.4,"x");
    let first = stuff.0;
    println!("first = {}", first);

    let (a,b,c) = stuff;
    println!("a = {}, b = {}, c = {}", a, b, c);

}
