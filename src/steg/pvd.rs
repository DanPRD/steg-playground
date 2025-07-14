use bitvec::{order::Msb0, vec::BitVec};
use image::{DynamicImage, GenericImageView};

use crate::{HEIGHT, WIDTH};

const INTERVALS: &[(i16, i16)] = &[
    (0,   7),
    (8,  15),
    (16, 31),
    (32, 63),
    (64,127),
    (128,255),
];

fn zigzags() -> Vec<usize> {
    let mut ret = Vec::with_capacity(WIDTH * HEIGHT);

    let mut direction_is_forward = true;

    for column in 0..HEIGHT {
        match direction_is_forward {
            true => ret.extend((0..WIDTH).map(|idx| idx + column*WIDTH)),
            false => ret.extend((0..WIDTH).rev().map(|idx| idx + column*WIDTH))
        }
        direction_is_forward = !direction_is_forward
    }

    ret
}

fn capacity(difference: i16) -> (u32, i16) {
    for &(r_min, r_max) in INTERVALS {
        if (r_min..=r_max).contains(&difference.abs()) {
            let size = r_max - r_min + 1;
            return (size.ilog2(), r_min);
        }
    }
    (1, 0)
}


pub fn embed_message(image: &mut [[u8; 4]], mut slug_bits: impl ExactSizeIterator<Item = u8>) -> (usize, usize) {

    for idxs in zigzags().chunks(2) {
        let pixel1 = image[idxs[0]];
        let pixel2 = image[idxs[1]];
        
        for colour_channel in 0..3 {
            let (col1, col2) = (pixel1[colour_channel] as i16, pixel2[colour_channel] as i16);
            let difference = col2 - col1;
            let (bit_capacity, r_min) = capacity(difference);
            let mut message_bits = 0;

            for idx in 0..bit_capacity {
                if let Some(bit) = slug_bits.next() {
                    message_bits |= bit << idx
                }
            }
            let new_difference = match difference >= 0 {
                true => r_min + message_bits as i16,
                false => -(r_min + message_bits as i16)
            };

            let total_difference = new_difference - difference;
            let floor = total_difference.div_euclid(2);
            let ceil = total_difference - floor;


            let (new_col1, new_col2) = match difference % 2 == 0     {
                true => {
                    (col1 - floor, col2 + ceil)
                }
                false => {
                    (col1 - ceil, col2 + floor)
                }
            };
            let new_col1 = new_col1.clamp(0, 255) as u8;
            let new_col2 = new_col2.clamp(0, 255) as u8;

            image[idxs[0]][colour_channel] = new_col1;
            image[idxs[1]][colour_channel] = new_col2;
        }

    }


    (0, 0)
}


pub fn solve_image(image: DynamicImage, bit_len: usize) -> String {
    let mut ret =  BitVec::<u8, Msb0>::with_capacity(bit_len);
    let pixels = image.pixels().collect::<Vec<_>>();
    let binding = zigzags();
    let mut idxs = binding.chunks(2);
    while ret.len() < bit_len {
        if let Some(idxs) = idxs.next() {
            let pixel1 = pixels[idxs[0]].2.0;
            let pixel2 = pixels[idxs[1]].2.0;
            for colour_channel in 0..3 {
                if ret.len() >= bit_len {
                    break
                }
                let difference = pixel2[colour_channel] as i16 - pixel1[colour_channel] as i16;
                let (bit_capacity, r_min) = capacity(difference);
                let message_bits = match difference >= 0 {
                    true => difference - r_min,
                    false => -difference - r_min
                };

                for idx in 0..bit_capacity {
                    ret.push(message_bits >> idx & 1 != 0);
                }

            }
        }
    }

    return String::from_utf8_lossy(&ret.into_vec()).into_owned()
}