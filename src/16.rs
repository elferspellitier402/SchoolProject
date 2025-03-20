use std::vec;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut sum = 0;
    for number in numbers.iter() {
        sum += number;
    }
    println!("The sum of the numbers is {}", sum);
}
