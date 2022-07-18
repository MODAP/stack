// import pylon API to seed pylon for camea
use pylon_cxx;
use anyhow::Result;

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
fn camera_initalization() {
    // create new instance
    let pylon = pylon_cxx::Pylon::new();
    // And then create camera
    let cam = Camera::new(&pylon);
    cam.debug();
}

#[tokio::test]
async fn camera_stream() -> anyhow::Result<()> {
    use tokio_stream::StreamExt;
    let pylon = pylon_cxx::Pylon::new();
    let mut camera = Camera::new(&pylon);
    camera.start_limited(2);
    let inner = camera.camera;
    tokio::pin!(inner);
    while let Some(grab_result) = inner.next().await {
    	println!("{}", grab_result.width().unwrap());
    	println!("{}", grab_result.height().unwrap());
    }
    panic!("AAA")
}
