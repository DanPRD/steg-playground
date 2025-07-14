use std::path::{Path, PathBuf};

use rand::SeedableRng;
use rand_xoshiro::Xoshiro128StarStar;


pub mod random;
pub mod steg;

pub(crate) const HEIGHT: usize = 800;
pub(crate) const WIDTH: usize = 800;
pub(crate) const OUTPUT_DIR: &str = "output";


fn rng_from_seed(seed: u32) -> Xoshiro128StarStar {
    rand_xoshiro::Xoshiro128StarStar::seed_from_u64(seed as u64)
}

pub fn save_image(pixel_buffer: &[[u8; 4]], filename: impl AsRef<Path>) {
    let mut path = PathBuf::from(OUTPUT_DIR);
    path.push(filename);
    path.set_extension("png");
    let raw: Vec<u8> = pixel_buffer.iter().flatten().map(|px| *px).collect();
    let _ = image::save_buffer(path, &raw, WIDTH as u32, HEIGHT as u32, image::ColorType::Rgba8);
}


