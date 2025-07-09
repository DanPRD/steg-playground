use std::path::Path;

use noise::{utils::{ColorGradient, ImageRenderer, NoiseMapBuilder, PlaneMapBuilder}, RidgedMulti, Simplex};
use rand::{Rng, SeedableRng};



const HEIGHT: usize = 800;
const WIDTH: usize = 800;
const LOWER_BOUND: f64 = -0.7;
const HIGHER_BOUND: f64 = 0.7;
const NUM_COLS: usize = 10;
const STEP: f64 = (HIGHER_BOUND - LOWER_BOUND) / (NUM_COLS as f64 - 1.0);

const POSITIONS: [f64; NUM_COLS] = {
    let mut arr = [0.0; NUM_COLS];
    let mut i = 0;
    while i < NUM_COLS {
        arr[i] = LOWER_BOUND + (i as f64) * STEP;
        i += 1;
    }
    arr
};


pub fn simplex_image(seed: u32) {
    let base_noise= RidgedMulti::<Simplex>::new(seed);

    let noise_map = PlaneMapBuilder::new(&base_noise)
        .set_size(WIDTH, HEIGHT)
        .set_x_bounds(LOWER_BOUND, HIGHER_BOUND)
        .set_y_bounds(LOWER_BOUND, HIGHER_BOUND)
        .build();

    ImageRenderer::new()
        .set_gradient(gen_col_gradient(seed))
        .render(&noise_map)
        .write_to_file(Path::new("output/random.png"));
}


fn gen_col_gradient(seed: u32) -> ColorGradient {
    let mut grad = ColorGradient::new();

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);

    for pos in POSITIONS {
        grad = grad.add_gradient_point(pos, [rng.random_range(0..255), rng.random_range(0..255), rng.random_range(0..255), 255]);
    }

    return grad;
}

