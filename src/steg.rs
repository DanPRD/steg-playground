#![allow(unused)]
use std::{fmt::Display, path::Path};

use bitvec::{order::Msb0, vec::BitVec};
use image::GenericImageView;
use rand::{distr::{Distribution, StandardUniform}, Rng, RngCore};

use crate::{random, rng_from_seed, HEIGHT, OUTPUT_DIR, WIDTH};

#[derive(Debug)]
pub enum StegMethod {
    LSB,
    RED,
    GREEN,
    BLUE,
    ALPHA,
    PVD,
    BPCS,
    DCT,
    DWT,
    DFT,
}

impl Distribution<StegMethod> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StegMethod {
        match rng.random_range(0..10) {
            0 => StegMethod::LSB,
            1 => StegMethod::RED,
            2 => StegMethod::GREEN,
            3 => StegMethod::BLUE,
            4 => StegMethod::ALPHA,
            5 => StegMethod::PVD,
            6 => StegMethod::BPCS,
            7 => StegMethod::DCT,
            8 => StegMethod::DWT,
            9 => StegMethod::DFT,
            _ => StegMethod::LSB
        }
    }
}

pub struct StegProblem {
    pub seed: u32,
    pub slug: String,
    pub method: StegMethod,
    pub pixel_offset: usize,
    pub image: Vec<[u8; 4]>
}

impl Display for StegProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "seed: {}\nslug: {}\nmethod: {:?}\noffset: {}", self.seed, self.slug, self.method, self.pixel_offset)
    }
}


const ANIMALS: [&str; 172] = ["ant","anteater","antelope","armadillo","auk","badger","bat","bear","beaver","bison","boar","buffalo","butterfly","camel","capybara","caribou","cat","caterpillar","cheetah","chimpanzee","chinchilla","chipmunk","civet","clam","cobra","cockroach","cougar","cow","coyote","crab","crane","crocodile","crow","deer","dingo","dog","dolphin","donkey","duck","eagle","earthworm","echidna","eel","elephant","elk","emu","falcon","ferret","finch","fish","flamingo","fly","fox","frog","gazelle","gecko","gerbil","giraffe","goat","goose","gorilla","grasshopper","hamster","hare","hawk","hedgehog","heron","hippopotamus","hornet","horse","hummingbird","hyena","ibis","iguana","impala","jaguar","jay","kangaroo","kingfisher","kiwi","koala","kudu","ladybug","lemur","leopard","lion","lizard","lobster","lynx","macaw","magpie","marmot","marten","meerkat","mink","mole","mongoose","monkey","moose","mosquito","mouse","mule","narwhal","newt","nightingale","ocelot","octopus","okapi","opossum","orangutan","ostrich","otter","owl","oyster","panda","panther","parrot","peacock","pelican","penguin","pheasant","pig","pigeon","porcupine","porpoise","quail","rabbit","racoon","ram","rat","raven","reindeer","rhinoceros","robin","salamander","salmon","sandpiper","scorpion","seahorse","shark","sheep","shrimp","skunk","sloth","snail","snake","sparrow","spider","squid","squirrel","starfish","stoat","stork","swan","tapir","termite","tiger","toad","trout","turkey","turtle","vulture","wallaby","walrus","wasp","weasel","whale","wolf","wolverine","worm","yak","zebra"];
const NUM_ANIMALS: usize = 4;

fn random_slug(seed: u32) -> String {
    let mut rng = rng_from_seed(seed);
    let mut ret = vec![];
    for _ in 0..NUM_ANIMALS {
        ret.push(ANIMALS[rng.random_range(0..ANIMALS.len())]);
    }
    return ret.join("-").to_string()
}

pub fn random_steg_challenge(seed: u32) -> StegProblem {
    let mut rng = rng_from_seed(seed);
    let slug = random_slug(rng.next_u32());
    let method: StegMethod = rng.random();
    let method = StegMethod::LSB;
    let mut image = random::simplex_image(seed);
    
    let mut slug_bits= BitVec::<_, Msb0>::from_slice(slug.as_bytes()).into_iter().map(|bit| bit as u8);
    let bit_len = slug_bits.len();

    let num_pixels = (bit_len + 2) / 3;
    let mut pixel_offset = rng.next_u64() as usize % ((WIDTH*HEIGHT) - num_pixels + 1 );

    let steg_image = match method {
        StegMethod::LSB => {

            for idx in 0..num_pixels {
                let mut pixel = image[idx + pixel_offset];

                if let Some(bit) = slug_bits.next() {
                    pixel[0] = (pixel[0] & !1) | bit;
                }

                if let Some(bit) = slug_bits.next() {
                    pixel[1] = (pixel[1] & !1) | bit;
                }

                if let Some(bit) = slug_bits.next() {
                    pixel[2] = (pixel[2] & !1) | bit;
                }

                image[idx + pixel_offset] = pixel;

            }
            image
        },
        _ => vec![]
    };
    
    return StegProblem {
        image: steg_image,
        pixel_offset,
        method,
        seed,
        slug
    }
}

pub fn solve_steg_challenge(seed: u32) -> Result<String, image::ImageError> {
    let mut rng = rng_from_seed(seed);
    let slug = random_slug(rng.next_u32());
    let method: StegMethod = rng.random();
    let method = StegMethod::LSB;
    let image = image::open(format!("{}/{}.png",OUTPUT_DIR, seed))?;
    
    let mut slug_bits= BitVec::<_, Msb0>::from_slice(slug.as_bytes()).into_iter().map(|bit| bit as u8);
    let bit_len = slug_bits.len();

    let num_pixels = (bit_len + 2) / 3;
    let mut pixel_offset = rng.next_u64() as usize % ((WIDTH*HEIGHT) - num_pixels + 1 );
    
    let mut ret =  BitVec::<u8, Msb0>::with_capacity(bit_len);
    
    let found_slug = match method {
        StegMethod::LSB => {
            for (_, _, rgb) in image.pixels().skip(pixel_offset).take(num_pixels) {
                let mut pixel: [u8; 4] = rgb.0;
                for col in &pixel[0..3] {
                    ret.push(col & 1 != 0);
                }
            }
            String::from_utf8_lossy(&ret[0..bit_len].to_bitvec().into_vec()).into_owned()
        }
        _ => String::new()
    };

    return Ok(found_slug);
}




