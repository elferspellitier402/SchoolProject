use std::cmp;

fn main() {
    let numbers: Vec<i32> = vec![4, 1, 8, 7, 0, 6, 9];
    let mut largest_number = numbers[0];

    for i in 1..numbers.len() {
        if numbers[i] > largest_number {
            largest_number = numbers[i];
        }
    }

    println!("The largest number is {}", largest_number);
}
