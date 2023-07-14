fn main() {
    let mut matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];

    for row in matrix.iter_mut() {
        for column in row.iter_mut() {
            *column = *column * 2;
            println!("{}\t", column);
        }
        println!();
    }

    // let numbers = [1,2,3,4,5,6,7,8,9,10];
    // let mut max: i32 = numbers[0];
    // let mut min: i32 = numbers[0];
    // let mut mean: f64 = 0.0;

    // for &i in numbers.iter(){
    //     if i >max {
    //         max = i;
    //     }
    //     else if i < min {
    //         min = i;
    //     }
    //     mean += i as f64;

    // }
    // mean /= numbers.len() as f64;

    // assert_eq!(max, 10);
    // assert_eq!(min, 1);
    // assert_eq!(mean, 5.5);
    // println!{"Test passed"}
}
