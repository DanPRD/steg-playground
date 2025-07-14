use bitvec::{order::Msb0, vec::BitVec};
use image::{DynamicImage, GenericImageView};
use rand::Rng;

use crate::{steg::Colour, HEIGHT, WIDTH};

pub(crate) fn embed(mut rng: impl Rng, colour: Colour, image: &mut [[u8; 4]], bits: impl ExactSizeIterator<Item = u8>) -> (usize, usize) {
    let mut pixel_offset = rng.next_u64() as usize % ((WIDTH*HEIGHT) - bits.len() + 1 );
    pixel_offset = pixel_offset - (pixel_offset % 8);
    let bit_layer_idx = rng.next_u32() as usize % 3;
    for (idx, bit) in bits.enumerate() {
        let mut pixel = image[idx + pixel_offset];

        pixel[colour as usize] = (pixel[colour as usize] & !(1 << bit_layer_idx)) | (bit << bit_layer_idx);
        image[idx + pixel_offset] = pixel;
    }

    return (pixel_offset, bit_layer_idx)
}

pub(crate) fn solve(mut rng: impl Rng, colour: Colour, image: DynamicImage, bit_len: usize) -> String {
    let mut ret =  BitVec::<u8, Msb0>::with_capacity(bit_len);
    let mut pixel_offset = rng.next_u64() as usize % ((WIDTH*HEIGHT) - bit_len + 1 );
    pixel_offset = pixel_offset - (pixel_offset % 8);
    let bit_layer_idx = rng.next_u32() as usize % 3;

    for (_, _, rgb) in image.pixels().skip(pixel_offset).take(bit_len) {
        let pixel: [u8; 4] = rgb.0;
        let bit = ((pixel[colour as usize] >> bit_layer_idx) & 1 ) != 0;
        ret.push(bit);
    } 

    String::from_utf8_lossy(&ret.into_vec()).into_owned()
}