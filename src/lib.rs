use std::path::{Path, PathBuf};

use rand::SeedableRng;
use rand_xoshiro::Xoshiro128StarStar;


pub mod random;
pub mod steg;
#[cfg(test)]
mod tests;

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

pub fn save_luma8_image(pixel_buffer: &[u8], filename: impl AsRef<Path>) {
    let mut path = PathBuf::from(OUTPUT_DIR);
    path.push(filename);
    path.set_extension("png");
    let _ = image::save_buffer(path, pixel_buffer, WIDTH as u32, HEIGHT as u32, image::ColorType::L8);
}

pub fn image_to_luma8(image: &[[u8; 4]]) -> Vec<u8> {
    image
    .iter()
    .map(|&[r, g, b, _a]| {
        ((299 * r as u32
        + 587 * g as u32
        + 114 * b as u32)
        / 1000) as u8
    })
    .collect()
}


