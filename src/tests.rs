use image::ImageError;

use crate::steg::{StegBuilder, StegMethod};



#[test]
fn builder_with_seed() {
    let chl = StegBuilder::new().with_seed(120).build();
    assert_eq!(chl.seed, 120)
}

#[test]
fn builder_with_method() {
    let chl = StegBuilder::new().with_method(StegMethod::LSB).build();
    assert_eq!(chl.method, StegMethod::LSB)
}

#[test]
fn lsb_solves() -> Result<(), ImageError> {
    let chl = StegBuilder::new().with_method(StegMethod::LSB).build();
    let solved_message = chl.solve()?;
    assert_eq!(solved_message, chl.slug);
    Ok(())
}

#[test]
fn red_layer_solves() -> Result<(), ImageError> {
    let chl = StegBuilder::new().with_method(StegMethod::RED).build();
    let solved_message = chl.solve()?;
    assert_eq!(solved_message, chl.slug);
    Ok(())
}

#[test]
fn green_layer_solves() -> Result<(), ImageError> {
    let chl = StegBuilder::new().with_method(StegMethod::GREEN).build();
    let solved_message = chl.solve()?;
    assert_eq!(solved_message, chl.slug);
    Ok(())
}

#[test]
fn blue_layer_solves() -> Result<(), ImageError> {
    let chl = StegBuilder::new().with_method(StegMethod::BLUE).build();
    let solved_message = chl.solve()?;
    assert_eq!(solved_message, chl.slug);
    Ok(())
}

#[test]
fn alpha_layer_solves() -> Result<(), ImageError> {
    let chl = StegBuilder::new().with_method(StegMethod::ALPHA).build();
    let solved_message = chl.solve()?;
    assert_eq!(solved_message, chl.slug);
    Ok(())
}

#[test]
fn pvd_solves() -> Result<(), ImageError> {
    let chl = StegBuilder::new().with_method(StegMethod::PVD).build();
    let solved_message = chl.solve()?;
    assert_eq!(solved_message, chl.slug);
    Ok(())
}

