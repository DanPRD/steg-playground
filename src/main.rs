use std::path::Path;

use rand::SeedableRng;
use rand_xoshiro::Xoshiro128StarStar;

use crate::steg::{random_steg_challenge, solve_steg_challenge};

mod random;
mod steg;

pub(crate) const HEIGHT: usize = 800;
pub(crate) const WIDTH: usize = 800;
pub(crate) const OUTPUT_DIR: &str = "output";

fn main() {
    let seed = rand::random();
    let challenge = random_steg_challenge(seed);
    save_image(&challenge.image, format!("{}/{}.png", OUTPUT_DIR, seed));
    println!("{}", challenge);
    match solve_steg_challenge(seed) {
        Ok(slug) => println!("Solved seed {}, found slug '{}'", seed, slug),
        Err(e) => println!("Error solving seed {}: {}", seed, e)
    }
}


fn rng_from_seed(seed: u32) -> Xoshiro128StarStar {
    rand_xoshiro::Xoshiro128StarStar::seed_from_u64(seed as u64)
}

pub fn save_image(pixel_buffer: &[[u8; 4]], path: impl AsRef<Path>) {
    let raw: Vec<u8> = pixel_buffer.iter().flatten().map(|px| *px).collect();
    let _ = image::save_buffer(path, &raw, WIDTH as u32, HEIGHT as u32, image::ColorType::Rgba8);
}


