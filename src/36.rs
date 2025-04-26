// Exercise 3: Rust Programming Language

fn main() {
    let mut secret_number = 42;

    // You can choose any integer value to make it more difficult or interesting.
    let difficulty_level: isize;
    if secret_number % 5 == 0 || secret_number % 7 == 0 {
        difficulty_level = 5;
    } else if secret_number % 3 == 0 || secret_number % 9 == 0 {
        difficulty_level = 6;
    } else if secret_number % 21 == 0 {
        difficulty_level = 7;
    } else {
        difficulty_level = 8;
    }

    println!("The {}-digit secret number is: {}", difficulty_level, secret_number);
}
