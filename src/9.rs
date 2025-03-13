import rand::{thread_rng, Rng};

fn main() {
    let r = thread_rng();
    let number: i32 = r.gen_range(0..50);
    println!("The number is {}", number);
}