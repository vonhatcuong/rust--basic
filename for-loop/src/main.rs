fn main() {
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    for (i,j) in letters.iter().enumerate() {
        println!("letter: {},{}", i,j);
        if i == 5 {
            break;
        }
    }
}
