use steg_playground::{save_image, steg::{random_steg_challenge, solve_steg_challenge}};
fn main() {
    let seed = rand::random();
    let challenge = random_steg_challenge(seed);
    save_image(&challenge.image, seed.to_string());
    println!("{}", challenge);
    match solve_steg_challenge(seed) {
        Ok(slug) => println!("Solved seed {}, found slug '{}'", seed, slug),
        Err(e) => println!("Error solving seed {}: {}", seed, e)
    }
}


