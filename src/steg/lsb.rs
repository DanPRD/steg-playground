use bitvec::{order::Msb0, vec::BitVec};
use rand::Rng;

use crate::{HEIGHT, WIDTH};

pub(crate) fn embed(mut rng: impl Rng, image: &mut [[u8; 4]], mut bits: impl ExactSizeIterator<Item = u8>) -> (usize, usize) {
    let num_pixels = (bits.len() + 2) / 3;
    let mut pixel_offset = rng.next_u64() as usize % ((WIDTH*HEIGHT) - num_pixels + 1 );
    pixel_offset = pixel_offset - (pixel_offset % 8);

    for idx in 0..num_pixels {
        let mut pixel = image[idx + pixel_offset];

        if let Some(bit) = bits.next() {
            pixel[0] = (pixel[0] & !1) | bit;
        }

        if let Some(bit) = bits.next() {
            pixel[1] = (pixel[1] & !1) | bit;
        }

        if let Some(bit) = bits.next() {
            pixel[2] = (pixel[2] & !1) | bit;
        }

        image[idx + pixel_offset] = pixel;

    }
    (pixel_offset, 0)
}

pub(crate) fn solve(mut rng: impl Rng, image: &[[u8; 4]], bit_len: usize) -> String {
    let mut ret =  BitVec::<u8, Msb0>::with_capacity(bit_len);
    let num_pixels = (bit_len + 2) / 3;
    let mut pixel_offset = rng.next_u64() as usize % ((WIDTH*HEIGHT) - num_pixels + 1 );
    pixel_offset = pixel_offset - (pixel_offset % 8);
    for pixel in image.iter().skip(pixel_offset).take(num_pixels) {
        for col in &pixel[0..3] {
            ret.push(col & 1 != 0);
        }
    }
    String::from_utf8_lossy(&ret[0..bit_len].to_bitvec().into_vec()).into_owned()
}