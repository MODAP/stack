// import pylon API to seed pylon for camea
use pylon_cxx;

// import everything from our end
use super::*;

//// Unit Tests ////

#[test]
/// A test for locale creation
fn locale_create() {
    let mut _loc = Locale::new((0.0,4000.0,0.0), 500);
}

#[test]
/// A test for locale updating
fn locale_update() {
    let mut loc = Locale::new((0.0,4000.0,0.0), 500);

    // DANGEROUS we unwrap b/c it returns a Result
    // but it could error with SystemTimeError
    // when Rust cannot access the hardware clock
    // and that case should be handled with `match`
    // instead of just wrapping

    loc.update((1.0, -9.8, 0.0),
               (0.0, 0.3, 0.5)).unwrap();
}

#[test]
/// A test for pylon camera initalization
fn camera_initalization() {
    // create new instance
    let pylon = pylon_cxx::Pylon::new();
    // And then create camera
    let cam = Camera::new(&pylon);
    cam.debug();
}

