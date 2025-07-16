use steg_playground::{save_image, steg::{StegBuilder, StegMethod}};

fn main() {
    let challenge = StegBuilder::new()
        .with_method(StegMethod::DWT)
        .build();
    save_image(&challenge.image, challenge.seed.to_string());
    println!("{}", challenge);
    match challenge.solve() {
        Ok(slug) => println!("Solved seed {}, found slug '{}'", challenge.seed, slug),
        Err(e) => println!("Error solving seed {}: {}", challenge.seed, e)
    }
}


