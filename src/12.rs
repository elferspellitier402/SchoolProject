use rand::{Rng, SeedableRng};

fn main() {
    let mut rng = rand::rngs::StdRng::from_seed(rand::SeedableRng::from_entropy());
    println!("Random number between 1 and 10: {}", rng.gen_range(1..=10));
}
