use std::fmt::Display;

use bitvec::{order::Msb0, vec::BitVec};
use rand::{distr::{Distribution, StandardUniform}, Rng, RngCore};

use crate::{random, rng_from_seed};

const ANIMALS: [&str; 172] = ["ant","anteater","antelope","armadillo","auk","badger","bat","bear","beaver","bison","boar","buffalo","butterfly","camel","capybara","caribou","cat","caterpillar","cheetah","chimpanzee","chinchilla","chipmunk","civet","clam","cobra","cockroach","cougar","cow","coyote","crab","crane","crocodile","crow","deer","dingo","dog","dolphin","donkey","duck","eagle","earthworm","echidna","eel","elephant","elk","emu","falcon","ferret","finch","fish","flamingo","fly","fox","frog","gazelle","gecko","gerbil","giraffe","goat","goose","gorilla","grasshopper","hamster","hare","hawk","hedgehog","heron","hippopotamus","hornet","horse","hummingbird","hyena","ibis","iguana","impala","jaguar","jay","kangaroo","kingfisher","kiwi","koala","kudu","ladybug","lemur","leopard","lion","lizard","lobster","lynx","macaw","magpie","marmot","marten","meerkat","mink","mole","mongoose","monkey","moose","mosquito","mouse","mule","narwhal","newt","nightingale","ocelot","octopus","okapi","opossum","orangutan","ostrich","otter","owl","oyster","panda","panther","parrot","peacock","pelican","penguin","pheasant","pig","pigeon","porcupine","porpoise","quail","rabbit","racoon","ram","rat","raven","reindeer","rhinoceros","robin","salamander","salmon","sandpiper","scorpion","seahorse","shark","sheep","shrimp","skunk","sloth","snail","snake","sparrow","spider","squid","squirrel","starfish","stoat","stork","swan","tapir","termite","tiger","toad","trout","turkey","turtle","vulture","wallaby","walrus","wasp","weasel","whale","wolf","wolverine","worm","yak","zebra"];
const NUM_ANIMALS: usize = 4;

mod pvd;
mod lsb;
mod layer;
mod dwt;

pub struct StegChallenge {
    pub seed: u32,
    pub slug: String,
    pub method: StegMethod,
    pub pixel_offset: usize,
    pub image: Vec<[u8; 4]>,
    pub layer_idx: usize
}

impl Display for StegChallenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "seed: {}\nslug: {}\nmethod: {:?}\noffset: {}\nlayer: {}", self.seed, self.slug, self.method, self.pixel_offset, self.layer_idx)
    }
}

impl StegChallenge {
    pub fn solve(&self) -> Result<String, image::ImageError> {
        solve_challenge(self.seed, Some(self.method), Some(&self.image))
    }
}

#[derive(Default)]
pub struct StegBuilder {
    seed: Option<u32>,
    method: Option<StegMethod>,
    offset: Option<usize>,
    layer: Option<usize>
}

impl StegBuilder {
    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_method(mut self, method: StegMethod) -> Self {
        self.method = Some(method);
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
    
    pub fn with_layer(mut self, layer: usize) -> Self {
        self.layer = Some(layer);
        self
    }

    pub fn build(self) -> StegChallenge {
        let seed = self.seed.unwrap_or(rand::random());
        return steg_challenge(seed, self.method)
    }

    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StegMethod {
    LSB, // LSB for R,G and B pixels
    RED, // for R only, random layer
    GREEN, // for G only, random layer
    BLUE, // for B only, random layer
    ALPHA, // hide something in alpha place
    APVD,
    PVD,
    BPCS,
    DCT,
    DWT,
    DFT,
}


#[derive(Clone, Copy, Debug)]
enum Colour {
    RED = 0,
    GREEN = 1,
    BLUE = 2,
    ALPHA = 3,
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


fn random_slug(seed: u32) -> String {
    let mut rng = rng_from_seed(seed);
    let mut ret = vec![];
    for _ in 0..NUM_ANIMALS {
        ret.push(ANIMALS[rng.random_range(0..ANIMALS.len())]);
    }
    return format!("DJP{{{}}}", ret.join("-").to_string());
}

fn steg_challenge(seed: u32, method: Option<StegMethod>) -> StegChallenge {
    let mut rng = rng_from_seed(seed);
    let slug = random_slug(rng.next_u32());
    let method = method.unwrap_or(rng_from_seed(seed).random());
    let mut image = random::simplex_image(seed);
    let slug_bits= BitVec::<_, Msb0>::from_slice(slug.as_bytes()).into_iter().map(|bit| bit as u8);

    let (pixel_offset, layer_idx) = match method {
        StegMethod::LSB => lsb::embed(rng, &mut image, slug_bits),
        StegMethod::RED => layer::embed(rng, Colour::RED, &mut image, slug_bits),
        StegMethod::GREEN => layer::embed(rng, Colour::GREEN, &mut image, slug_bits),    
        StegMethod::BLUE => layer::embed(rng, Colour::BLUE, &mut image, slug_bits),
        StegMethod::ALPHA => layer::embed(rng, Colour::ALPHA, &mut image, slug_bits),
        StegMethod::PVD => pvd::embed(&mut image, slug_bits),
        StegMethod::DWT => dwt::embed(&image),
        _ => (0, 0)
    };
    
    return StegChallenge {
        image,
        pixel_offset,
        method,
        seed,
        slug,
        layer_idx
    }
}

pub fn solve_challenge(seed: u32, method: Option<StegMethod>, image: Option<&[[u8; 4]]>) -> Result<String, image::ImageError>
{
    let mut rng = rng_from_seed(seed);
    let slug = random_slug(rng.next_u32());
    let method = method.unwrap_or_else(|| rng_from_seed(seed).random());
    let buf;
    let image = match image {
        Some(img) => img,
        None => {
            buf = random::simplex_image(seed);
            &buf
        }
    };
    let bit_len = slug.len() * 8;
    
    let found_slug = match method {
        StegMethod::LSB => lsb::solve(rng, image, bit_len),
        StegMethod::RED => layer::solve(rng, Colour::RED, image, bit_len),
        StegMethod::GREEN => layer::solve(rng, Colour::GREEN, image, bit_len),
        StegMethod::BLUE => layer::solve(rng, Colour::BLUE, image, bit_len),
        StegMethod::ALPHA => layer::solve(rng, Colour::ALPHA, image, bit_len),
        StegMethod::PVD => pvd::solve(image, bit_len),
        StegMethod::DWT => dwt::solve(),
        _ => String::new()
    };

    return Ok(found_slug);
}




